//! A runtime context. Contains state and messages.

use flume::{Receiver, RecvError, SendError, Sender};

/// The `Context` type keeps track of system-wide state. Specifically, it owns
/// the shutdown feature.
pub struct Context {
    shutdown_sender: Sender<()>,
    shutdown_receiver: Receiver<()>,

    emergency_shutdown_sender: Sender<()>,
    emergency_shutdown_receiver: Receiver<()>,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {
    pub fn new() -> Self {
        let (shutdown_sender, shutdown_receiver) = flume::bounded(1);
        let (emergency_shutdown_sender, emergency_shutdown_receiver) = flume::bounded(1);

        Self {
            shutdown_sender,
            shutdown_receiver,
            emergency_shutdown_sender,
            emergency_shutdown_receiver,
        }
    }

    pub async fn shutdown(&self) -> Result<()> {
        self.shutdown_sender.send_async(()).await?;
        Ok(())
    }

    pub async fn wait_for_shutdown(&self) -> Result<()> {
        self.shutdown_receiver.recv_async().await?;
        Ok(())
    }

    pub async fn emergency_shutdown(&self) -> Result<()> {
        self.emergency_shutdown_sender.send_async(()).await?;
        Ok(())
    }

    pub async fn wait_for_emergency_shutdown(&self) -> Result<()> {
        self.emergency_shutdown_receiver.recv_async().await?;
        Ok(())
    }
}

#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum ContextError<T: std::fmt::Debug> {
    #[error(transparent)]
    #[diagnostic(code(mekena::context::receive_error))]
    ReceiveError(#[from] RecvError),

    #[error(transparent)]
    #[diagnostic(code(mekena::context::send_error))]
    SendError(#[from] SendError<T>),
}

type Result<T> = std::result::Result<T, ContextError<()>>;
