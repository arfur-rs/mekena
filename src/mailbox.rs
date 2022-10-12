use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crossbeam::sync::ShardedLock;
use flume::{Receiver, Sender};

use crate::message::Message;

pub struct Mailbox {
    sender: Sender<Box<dyn Any + Send + Sync>>,
    receiver: Receiver<Box<dyn Any + Send + Sync>>,
}

unsafe impl Send for Mailbox {}
unsafe impl Sync for Mailbox {}

impl Mailbox {
    pub fn new() -> Self {
        let (sender, receiver) = flume::unbounded();
        Self { sender, receiver }
    }

    pub async fn send<M: Message + 'static>(&self, message: M) -> Result<(), MailboxError> {
        self.sender.send_async(Box::new(message)).await?;

        Ok(())
    }

    pub async fn recv<M: Message + 'static>(&self) -> Result<Option<Box<M>>, MailboxError> {
        let t = TypeId::of::<M>();

        loop {
            let received = self.receiver.recv_async().await?;
            match received.downcast::<M>() {
                Ok(x) => return Ok(Some(x)),
                _ => (),
            };
        }
    }
}

#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum MailboxError {
    #[error(transparent)]
    #[diagnostic(code(mekena::mailbox::could_not_send))]
    SendError(#[from] flume::SendError<Box<dyn Any + Send + Sync>>),
    #[error(transparent)]
    #[diagnostic(code(mekena::mailbox::could_not_recv))]
    RecvError(#[from] flume::RecvError),
    #[error("Downcast error. Something may be wrong with Mekena itself.")]
    #[diagnostic(code(mekena::mailbox::could_not_downcast))]
    DowncastError(Box<dyn Any + Send + Sync>),
}
