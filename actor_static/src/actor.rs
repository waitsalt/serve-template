use std::{error::Error, fmt, time::Duration};
use tokio::sync::{mpsc, oneshot};
use tokio_util::sync::CancellationToken;

const BATCH_SIZE: usize = 16;

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

pub trait Actor: Send + Sized + 'static {
    type Message: Send + 'static;
    type Error: Error + Send + Sync + 'static;

    fn handle(
        &mut self,
        msg: Self::Message,
        _ctx: &Context<Self>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

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

    fn start(mut self, config: ActorConfig) -> Addr<Self> {
        let batch_size = config.batch_size.max(1);
        let (tx, mut rx) = mpsc::channel(config.channel_size);
        let cancel_token = CancellationToken::new();
        let token_for_task = cancel_token.clone();

        let weak_sender = tx.downgrade();
        let weak_addr = WeakAddr {
            sender: weak_sender,
            cancel_token: cancel_token.clone(),
        };

        tokio::spawn(async move {
            let context = Context {
                weak_addr: weak_addr.clone(),
            };

            if let Err(e) = self.on_start(&context).await {
                tracing::error!(%e,"Actor 启动时发生错误");
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
                            if let Err(e) = self.handle(msg, &context).await {
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

pub struct Addr<A: Actor> {
    sender: mpsc::Sender<A::Message>,
    cancel_token: CancellationToken,
}
#[derive(Debug)]
pub enum ActorError<E> {
    Closed,
    Full,
    Timeout,
    AskSendClosed,
    AskRecvDropped,
    AskTimeout,
    Handler(E),
}

impl<E> fmt::Display for ActorError<E>
where
    E: fmt::Display,
{
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
            ActorError::Handler(err) => write!(f, "{err}"),
        }
    }
}

impl<E> Error for ActorError<E> where E: Error + 'static {}

impl<A: Actor> fmt::Debug for Addr<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Addr")
            .field(
                "sender",
                &format_args!("mpsc::Sender<{}>", std::any::type_name::<A::Message>()),
            )
            .field("cancel_token", &self.cancel_token)
            .finish()
    }
}

impl<A: Actor> Clone for Addr<A> {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            cancel_token: self.cancel_token.clone(),
        }
    }
}

impl<A> Addr<A>
where
    A: Actor,
{
    pub async fn send(&self, msg: A::Message) -> Result<(), ActorError<A::Error>> {
        self.sender
            .send(msg)
            .await
            .map_err(|_| ActorError::Closed)
    }

    pub fn try_send(&self, msg: A::Message) -> Result<(), ActorError<A::Error>> {
        self.sender
            .try_send(msg)
            .map_err(|e| match e {
                mpsc::error::TrySendError::Full(_) => ActorError::Full,
                mpsc::error::TrySendError::Closed(_) => ActorError::Closed,
            })
    }

    pub async fn send_timeout(
        &self,
        msg: A::Message,
        timeout: Duration,
    ) -> Result<(), ActorError<A::Error>> {
        self.sender
            .send_timeout(msg, timeout)
            .await
            .map_err(|e| match e {
                mpsc::error::SendTimeoutError::Timeout(_) => ActorError::Timeout,
                mpsc::error::SendTimeoutError::Closed(_) => ActorError::Closed,
            })
    }

    pub async fn ask<R, F>(&self, f: F) -> Result<R, ActorError<A::Error>>
    where
        R: Send + 'static,
        F: FnOnce(oneshot::Sender<R>) -> A::Message,
    {
        let (tx, rx) = oneshot::channel();
        let msg = f(tx);
        self.send(msg)
            .await
            .map_err(|_| ActorError::AskSendClosed)?;
        rx.await
            .map_err(|_| ActorError::AskRecvDropped)
    }

    pub async fn ask_timeout<R, F>(&self, f: F, timeout: Duration) -> Result<R, ActorError<A::Error>>
    where
        R: Send + 'static,
        F: FnOnce(oneshot::Sender<R>) -> A::Message,
    {
        tokio::time::timeout(timeout, self.ask(f))
            .await
            .map_err(|_| ActorError::AskTimeout)?
    }

    pub fn cancel(&self) {
        self.cancel_token.cancel();
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancel_token.is_cancelled()
    }

    pub fn downgrade(&self) -> WeakAddr<A> {
        WeakAddr {
            sender: self.sender.downgrade(),
            cancel_token: self.cancel_token.clone(),
        }
    }
}

pub struct WeakAddr<A: Actor> {
    sender: mpsc::WeakSender<A::Message>,
    cancel_token: CancellationToken,
}

impl<A: Actor> fmt::Debug for WeakAddr<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WeakAddr")
            .field(
                "sender",
                &format_args!("mpsc::WeakSender<{}>", std::any::type_name::<A::Message>()),
            )
            .field("cancel_token", &self.cancel_token)
            .finish()
    }
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
pub struct Context<A>
where
    A: Actor,
{
    weak_addr: WeakAddr<A>,
}

impl<A> Context<A>
where
    A: Actor,
{
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
