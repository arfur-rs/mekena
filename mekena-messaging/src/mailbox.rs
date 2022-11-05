//! The main, multidirectional, MPMC, strongly-typed messaging system for
//! Mekena.
//!
//! It depends mainly on [`flume::unbounded`] channels, allowing a
//! flexible and quite fast MPMC system. It stores the data as
//! [`std::any::Any`], using `downcast` to (try) to reconstruct them.
//!
//! [`Mailbox`] implements Send/Sync, so it can safely be sent across threads.

use std::any::Any;

use flume::{Receiver, Sender};

use crate::message::Message;

// TODO: double check that this is actually efficient. If not, try a DashMap +
// TypeId or String ID implementation.

/// The main, multidirectional, MPMC, strongly-typed messager for Mekena. A
/// collection of [`flume`] channels, it stores any type T as
/// [`std::any::Any`], but checks to make sure your type is right with
/// `downcast`.
#[derive(Debug)]
pub struct Mailbox {
    sender: Sender<Box<dyn Any + Send + Sync>>,
    receiver: Receiver<Box<dyn Any + Send + Sync>>,
}

unsafe impl Send for Mailbox {}
unsafe impl Sync for Mailbox {}

impl Mailbox {
    /// Construct a new, blank [`Mailbox`].
    pub fn new() -> Self {
        let (sender, receiver) = flume::unbounded();
        Self { sender, receiver }
    }

    /// Send any message: [`Message`] to the mailbox.
    pub async fn send<M: Message + 'static>(&self, message: M) -> Result<(), MailboxError> {
        self.sender.send_async(Box::new(message)).await?;

        Ok(())
    }

    /// Asynchronously wait for a new message with type M: [`Message`].
    pub async fn recv<M: Message + 'static>(&self) -> Result<Box<M>, MailboxError> {
        loop {
            let received = self.receiver.recv_async().await?;
            match received.downcast::<M>() {
                Ok(x) => return Ok(x),
                _ => (),
            };
        }
    }
}

#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum MailboxError {
    #[error(transparent)]
    #[diagnostic(code(mekena_messaging::mailbox::could_not_send))]
    SendError(#[from] flume::SendError<Box<dyn Any + Send + Sync>>),
    #[error(transparent)]
    #[diagnostic(code(mekena_messaging::mailbox::could_not_recv))]
    RecvError(#[from] flume::RecvError),
    #[error("Downcast error. Something may be wrong with Mekena itself.")]
    #[diagnostic(code(mekena_messaging::mailbox::could_not_downcast))]
    DowncastError(Box<dyn Any + Send + Sync>),
}
