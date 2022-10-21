use std::any::Any;

use dashmap::mapref::one::{Ref, RefMut};
use mekena_messaging::{
    mailbox::Mailbox,
    prelude::{MailboxError, Message},
};
use mekena_state::StateManager;
use mekena_util::shutdown::ShutdownManager;

pub struct Context {
    mailbox: Mailbox,
    state: StateManager,
    shutdown: ShutdownManager,
}

impl Context {
    pub fn new() -> Self {
        Self {
            mailbox: Mailbox::new(),
            state: StateManager::new(),
            shutdown: ShutdownManager::new(),
        }
    }

    /// Send any message: [`Message`] to the mailbox.
    pub async fn send<M: Message + 'static>(&self, message: M) -> Result<(), ContextError> {
        self.mailbox.send(message).await.map_err(ContextError::from)
    }

    /// Asynchronously wait for a new message with type M: [`Message`].
    pub async fn recv<M: Message + 'static>(&self) -> Result<Box<M>, ContextError> {
        self.mailbox.recv::<M>().await.map_err(ContextError::from)
    }

    /// Inserts a key and a value into the map. Returns the old value associated
    /// with the key if there was one.
    ///
    /// **Locking behaviour: May deadlock if called when holding any sort of
    /// reference into the map.**. Unfortunately, this is inherited from
    /// [`dashmap`].
    pub fn insert<V: 'static + Send + Sync>(&self, key: String, value: V) -> Option<Box<V>> {
        self.state.insert(key, value)
    }

    /// Get a immutable reference to an entry in the map
    ///
    /// **Locking behaviour: May deadlock if called when holding a mutable
    /// reference into the map.** Unfortunately, this is inherited from
    /// [`dashmap`].
    pub fn get(&self, key: String) -> Option<Ref<String, Box<dyn Any + Send + Sync>>> {
        self.state.get(key)
    }

    /// Get a mutable reference to an entry in the map
    ///
    /// **Locking behaviour: May deadlock if called when holding any sort of
    /// reference into the map.** Unfortunately, this is inherited from
    /// [`dashmap`].
    pub fn get_mut(&self, key: String) -> Option<RefMut<String, Box<dyn Any + Send + Sync>>> {
        self.state.get_mut(key)
    }

    pub async fn shutdown(&self) {
        self.shutdown.shutdown().await
    }

    pub async fn await_shutdown(&self) {
        self.shutdown.await_shutdown().await
    }
}

#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum ContextError {
    #[error(transparent)]
    MailboxError(#[from] MailboxError),
}
