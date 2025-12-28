use anyhow::{Result, anyhow};
use std::{pin::Pin, sync::Arc, time::Duration};
use tokio::sync::{Mutex, mpsc, oneshot};

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub trait Message: Send + 'static {
    type Result: Send;
}

pub trait Actor: Send + Sized + 'static {
    fn on_start(&mut self) -> impl Future<Output = Result<()>> + Send {
        async {
            tracing::trace!("Actor 启动");
            Ok(())
        }
    }
    fn on_stop(&mut self) -> impl Future<Output = Result<()>> + Send {
        async {
            tracing::trace!("Actor 停止");
            Ok(())
        }
    }

    fn start(mut self, size: usize) -> Addr<Self> {
        let (tx, mut rx) = mpsc::channel::<Box<dyn Envelope<Self>>>(size);
        let (stop_tx, mut stop_rx) = oneshot::channel();
        let stop_sender = Arc::new(Mutex::new(Some(stop_tx)));

        tokio::spawn(async move {
            if let Err(e) = self.on_start().await {
                tracing::error!("启动 Actor 时发生错误: {}", e);
                return;
            }

            loop {
                tokio::select! {
                    res = rx.recv() => {
                        match res {
                            Some(msg) => {}
                            None => {}
                        }
                    }

                    _ = &mut stop_rx => {
                        break;
                    }
                }
            }

            if let Err(e) = self.on_stop().await {
                tracing::error!("停止 Actor 时发生错误: {}", e);
            }
        });

        Addr {
            sender: tx,
            stop_sender,
        }
    }
}

pub trait Handler<M: Message>: Actor {
    fn handle(&mut self, msg: M) -> impl Future<Output = M::Result> + Send;
}

pub trait Envelope<A: Actor>: Send {
    fn handle(self: Box<Self>, actor: &mut A) -> BoxFuture<'_, ()>;
}

struct MsgEnvelope<M: Message> {
    msg: M,
    tx: Option<oneshot::Sender<M::Result>>,
}

impl<A, M> Envelope<A> for MsgEnvelope<M>
where
    A: Actor + Handler<M>,
    M: Message,
{
    fn handle(self: Box<Self>, actor: &mut A) -> BoxFuture<'_, ()> {
        Box::pin(async move {
            let result = actor.handle(self.msg).await;

            if let Some(tx) = self.tx {
                let _ = tx.send(result);
            }
        })
    }
}

#[derive(Debug)]
pub struct Addr<A: Actor> {
    sender: mpsc::Sender<Box<dyn Envelope<A>>>,
    stop_sender: Arc<Mutex<Option<oneshot::Sender<()>>>>,
}

impl<A: Actor> Clone for Addr<A> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            stop_sender: self.stop_sender.clone(),
        }
    }
}

impl<A: Actor> Addr<A> {
    pub async fn call<M>(&self, msg: M) -> Result<M::Result>
    where
        A: Handler<M>,
        M: Message,
    {
        let (tx, rx) = oneshot::channel();

        let env = MsgEnvelope { msg, tx: Some(tx) };

        self.sender
            .send(Box::new(env))
            .await
            .map_err(|_| anyhow!("Actor 被关闭"))?;

        rx.await.map_err(|_| anyhow!("Actor 无法获得响应"))
    }

    pub async fn call_with_timeout<M>(&self, msg: M, timeout: Duration) -> Result<M::Result>
    where
        A: Handler<M>,
        M: Message,
    {
        let (tx, rx) = oneshot::channel();

        let env = MsgEnvelope { msg, tx: Some(tx) };

        self.sender
            .send(Box::new(env))
            .await
            .map_err(|_| anyhow!("Actor 被关闭"))?;

        tokio::time::timeout(timeout, rx)
            .await
            .map_err(|_| anyhow!("Actor 响应超时"))?
            .map_err(|_| anyhow!("Actor 无法获得响应"))
    }

    pub async fn send<M>(&self, msg: M) -> Result<()>
    where
        A: Handler<M>,
        M: Message,
    {
        let env = MsgEnvelope { msg, tx: None };

        self.sender
            .send(Box::new(env))
            .await
            .map_err(|_| anyhow!("Actor 被关闭"))
    }

    pub fn try_send<M>(&self, msg: M) -> Result<()>
    where
        A: Handler<M>,
        M: Message,
    {
        let env = MsgEnvelope { msg, tx: None };

        self.sender
            .try_send(Box::new(env))
            .map_err(|_| anyhow!("Actor 通道满了/被关闭"))
    }

    pub async fn stop(&self) -> Result<()> {
        let mut guard = self.stop_sender.lock().await;

        if let Some(stop_tx) = guard.take() {
            let _ = stop_tx.send(());
            Ok(())
        } else {
            Err(anyhow!("Actor 已被命令停止"))
        }
    }
}
