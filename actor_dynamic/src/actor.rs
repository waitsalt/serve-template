use anyhow::{Result, anyhow};
use std::{pin::Pin, time::Duration};
use tokio::sync::{mpsc, oneshot};
use tokio_util::sync::CancellationToken;

const BATCH_SIZE: usize = 16;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub trait Message: Send + 'static {
    type Result: Send;
}

pub trait Actor: Send + Sized + 'static {
    fn on_start(&mut self, _ctx: &Context<Self>) -> impl Future<Output = Result<()>> + Send {
        async {
            tracing::trace!("Actor 启动");
            Ok(())
        }
    }

    fn on_stop(&mut self, _ctx: &Context<Self>) -> impl Future<Output = Result<()>> + Send {
        async {
            tracing::trace!("Actor 停止");
            Ok(())
        }
    }

    fn start(mut self, size: usize) -> Addr<Self> {
        let (tx, mut rx) = mpsc::channel::<Box<dyn Envelope<Self>>>(size);
        let cancel_token = CancellationToken::new();
        let token_for_task = cancel_token.clone();

        let weak_addr = WeakAddr {
            sender: tx.downgrade(),
            cancel_token: token_for_task.clone(),
        };
        let context = Context { weak_addr };

        tokio::spawn(async move {
            if let Err(e) = self.on_start(&context).await {
                tracing::error!(%e, "Actor 启动时发生错误");
                return;
            }

            let mut buffer = Vec::with_capacity(BATCH_SIZE);
            let mut close_loop = false;

            loop {
                tokio::select! {
                    biased;

                    _ = token_for_task.cancelled() => {
                        tracing::trace!("Actor 收到停止信号");
                        break;
                    }
                    count = rx.recv_many(&mut buffer, BATCH_SIZE) => {
                        if count == 0 {
                            tracing::trace!("Actor 消息通道已关闭");
                            close_loop = true;
                        }

                        for msg in buffer.drain(..) {
                            if token_for_task.is_cancelled() {
                                break;
                            }

                            if let Err(e) = msg.handle(&mut self, &context).await {
                                tracing::error!(%e, "Actor 处理消息错误");
                            }
                        }

                        if close_loop {
                            break;
                        }
                    }
                }
            }

            if let Err(e) = self.on_stop(&context).await {
                tracing::error!(%e, "Actor 停止时发生错误");
            }
        });

        Addr {
            sender: tx,
            cancel_token,
        }
    }
}

pub trait Handler<M: Message>: Actor {
    fn handle(&mut self, msg: M, ctx: &Context<Self>) -> impl Future<Output = M::Result> + Send;
}

pub trait Envelope<A: Actor>: Send {
    fn handle<'a>(
        self: Box<Self>,
        actor: &'a mut A,
        ctx: &'a Context<A>,
    ) -> BoxFuture<'a, Result<()>>;
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
    fn handle<'a>(
        self: Box<Self>,
        actor: &'a mut A,
        ctx: &'a Context<A>,
    ) -> BoxFuture<'a, Result<()>> {
        Box::pin(async move {
            let result = actor.handle(self.msg, ctx).await;

            if let Some(tx) = self.tx {
                let _ = tx.send(result);
            }

            Ok(())
        })
    }
}

#[derive(Debug)]
pub struct Addr<A: Actor> {
    sender: mpsc::Sender<Box<dyn Envelope<A>>>,
    cancel_token: CancellationToken,
}

impl<A: Actor> Clone for Addr<A> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            cancel_token: self.cancel_token.clone(),
        }
    }
}

impl<A: Actor> Addr<A> {
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

    pub async fn ask<M>(&self, msg: M) -> Result<M::Result>
    where
        A: Handler<M>,
        M: Message,
    {
        let (tx, rx) = oneshot::channel();

        let env = MsgEnvelope { msg, tx: Some(tx) };

        self.sender
            .send(Box::new(env))
            .await
            .map_err(|e| anyhow!("Actor 发送消息失败: {:#}", e))?;

        rx.await
            .map_err(|e| anyhow!("等待响应失败(Actor可能已丢弃请求): {:#}", e))
    }

    pub async fn ask_with_timeout<M>(&self, msg: M, timeout: Duration) -> Result<M::Result>
    where
        A: Handler<M>,
        M: Message,
    {
        tokio::time::timeout(timeout, self.ask(msg))
            .await
            .map_err(|_| anyhow!("Actor 请求超时"))?
    }

    pub fn stop(&self) {
        self.cancel_token.cancel();
    }

    pub fn is_stopped(&self) -> bool {
        self.cancel_token.is_cancelled()
    }
}

#[derive(Debug)]
pub struct WeakAddr<A: Actor> {
    sender: mpsc::WeakSender<Box<dyn Envelope<A>>>,
    cancel_token: CancellationToken,
}

impl<A: Actor> Clone for WeakAddr<A> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            cancel_token: self.cancel_token.clone(),
        }
    }
}

impl<A: Actor> WeakAddr<A> {
    pub fn upgrade(&self) -> Option<Addr<A>> {
        if let Some(sender) = self.sender.upgrade() {
            Some(Addr {
                sender,
                cancel_token: self.cancel_token.clone(),
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Context<A: Actor> {
    weak_addr: WeakAddr<A>,
}

impl<A: Actor> Context<A> {
    pub fn addr(&self) -> Option<Addr<A>> {
        self.weak_addr.upgrade()
    }

    pub fn weak_addr(&self) -> WeakAddr<A> {
        self.weak_addr.clone()
    }

    pub fn stop(&self) {
        self.weak_addr.cancel_token.cancel();
    }
}
