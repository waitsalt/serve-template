use std::{error::Error, fmt, pin::Pin, time::Duration};
use tokio::sync::{mpsc, oneshot};
use tokio_util::sync::CancellationToken;

const BATCH_SIZE: usize = 16;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[derive(Debug, Clone, Copy)]
pub struct ActorConfig {
    pub channel_size: usize,
    pub batch_size: usize,
}

impl ActorConfig {
    pub fn new(channel_size: usize) -> Self {
        Self {
            channel_size,
            batch_size: BATCH_SIZE,
        }
    }

    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size.max(1);
        self
    }
}

pub trait Message: Send + 'static {
    type Result: Send;
}

pub trait Actor: Send + Sized + 'static {
    type Error: Error + Send + Sync + 'static;

    fn on_start(
        &mut self,
        _ctx: &Context<Self>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async {
            tracing::trace!("Actor 启动");
            Ok(())
        }
    }

    fn on_stop(
        &mut self,
        _ctx: &Context<Self>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send {
        async {
            tracing::trace!("Actor 停止");
            Ok(())
        }
    }

    fn start(self, channel_size: usize) -> Addr<Self> {
        self.start_with_config(ActorConfig::new(channel_size))
    }

    fn start_with_config(mut self, config: ActorConfig) -> Addr<Self> {
        let batch_size = config.batch_size.max(1);
        let (tx, mut rx) = mpsc::channel::<Box<dyn Envelope<Self>>>(config.channel_size);
        let cancel_token = CancellationToken::new();
        let token_for_task = cancel_token.clone();

        let weak_addr = WeakAddr {
            sender: tx.downgrade(),
            cancel_token: cancel_token.clone(),
        };

        tokio::spawn(async move {
            let context = Context {
                weak_addr: weak_addr.clone(),
            };

            if let Err(e) = self.on_start(&context).await {
                tracing::error!(%e, "Actor 启动时发生错误");
                return;
            }

            let mut buffer = Vec::with_capacity(batch_size);
            let mut close_loop = false;

            loop {
                tokio::select! {
                    _ = token_for_task.cancelled() => {
                        tracing::trace!("Actor 收到停止信号");
                        break;
                    }
                    count = rx.recv_many(&mut buffer, batch_size) => {
                        if count == 0 {
                            tracing::trace!("Actor 消息通道已关闭");
                            close_loop = true;
                        }

                        if token_for_task.is_cancelled() {
                            tracing::trace!("Actor 收到停止信号");
                            break;
                        }

                        for msg in buffer.drain(..) {
                            msg.handle(&mut self, &context).await;
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
    ) -> BoxFuture<'a, ()>;
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
    ) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            let result = actor.handle(self.msg, ctx).await;

            if let Some(tx) = self.tx {
                let _ = tx.send(result);
            }
        })
    }
}

#[derive(Debug)]
pub struct Addr<A: Actor> {
    sender: mpsc::Sender<Box<dyn Envelope<A>>>,
    cancel_token: CancellationToken,
}

#[derive(Debug)]
pub enum ActorError {
    Closed,
    Full,
    Timeout,
    AskSendClosed,
    AskRecvDropped,
    AskTimeout,
}

impl fmt::Display for ActorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActorError::Closed => write!(f, "Actor 发送消息失败: 通道已关闭"),
            ActorError::Full => write!(f, "Actor 发送消息失败: 通道已满"),
            ActorError::Timeout => write!(f, "Actor 发送消息失败: 超时"),
            ActorError::AskSendClosed => write!(f, "Actor 发送消息失败: 通道已关闭"),
            ActorError::AskRecvDropped => {
                write!(f, "Actor 等待响应失败(发送端被丢弃/未发送)")
            }
            ActorError::AskTimeout => write!(f, "Actor 请求超时"),
        }
    }
}

impl Error for ActorError {}

impl<A: Actor> Clone for Addr<A> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            cancel_token: self.cancel_token.clone(),
        }
    }
}

impl<A: Actor> Addr<A> {
    pub async fn send<M>(&self, msg: M) -> Result<(), ActorError>
    where
        A: Handler<M>,
        M: Message,
    {
        let env = MsgEnvelope { msg, tx: None };

        self.sender
            .send(Box::new(env))
            .await
            .map_err(|_| ActorError::Closed)
    }

    pub fn try_send<M>(&self, msg: M) -> Result<(), ActorError>
    where
        A: Handler<M>,
        M: Message,
    {
        let env = MsgEnvelope { msg, tx: None };

        self.sender
            .try_send(Box::new(env))
            .map_err(|e| match e {
                mpsc::error::TrySendError::Full(_) => ActorError::Full,
                mpsc::error::TrySendError::Closed(_) => ActorError::Closed,
            })
    }

    pub async fn send_timeout<M>(&self, msg: M, timeout: Duration) -> Result<(), ActorError>
    where
        A: Handler<M>,
        M: Message,
    {
        let env = MsgEnvelope { msg, tx: None };

        self.sender
            .send_timeout(Box::new(env), timeout)
            .await
            .map_err(|e| match e {
                mpsc::error::SendTimeoutError::Timeout(_) => ActorError::Timeout,
                mpsc::error::SendTimeoutError::Closed(_) => ActorError::Closed,
            })
    }

    pub async fn ask<M>(&self, msg: M) -> Result<M::Result, ActorError>
    where
        A: Handler<M>,
        M: Message,
    {
        let (tx, rx) = oneshot::channel();

        let env = MsgEnvelope { msg, tx: Some(tx) };

        self.sender
            .send(Box::new(env))
            .await
            .map_err(|_| ActorError::AskSendClosed)?;

        rx.await
            .map_err(|_| ActorError::AskRecvDropped)
    }

    pub async fn ask_timeout<M>(
        &self,
        msg: M,
        timeout: Duration,
    ) -> Result<M::Result, ActorError>
    where
        A: Handler<M>,
        M: Message,
    {
        tokio::time::timeout(timeout, self.ask(msg))
            .await
            .map_err(|_| ActorError::AskTimeout)?
    }

    pub fn cancel(&self) {
        self.cancel_token.cancel();
    }

    pub fn is_cancelled(&self) -> bool {
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

    pub fn cancel(&self) {
        self.weak_addr.cancel_token.cancel();
    }
}
