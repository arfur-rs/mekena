//! Strongly-typed, very fast, asynchronous state.
//!
//! Stores state as a DashMap<dyn Any + Send + Sync>, but uses `downcast` to
//! ensure safe typing.

use std::any::Any;

use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};

pub struct StateManager {
    states: DashMap<String, Box<dyn Any + Send + Sync>>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            states: DashMap::new(),
        }
    }

    /// Inserts a key and a value into the map. Returns the old value associated
    /// with the key if there was one.
    ///
    /// **Locking behaviour: May deadlock if called when holding any sort of
    /// reference into the map.**. Unfortunately, this is inherited from
    /// [`dashmap`].
    pub fn insert<V: 'static + Send + Sync>(&self, key: String, value: V) -> Option<Box<V>> {
        self.states
            .insert(key, Box::new(value))
            .map(|x| x.downcast::<V>().unwrap())
    }

    /// Get a immutable reference to an entry in the map
    ///
    /// **Locking behaviour: May deadlock if called when holding a mutable
    /// reference into the map.** Unfortunately, this is inherited from
    /// [`dashmap`].
    pub fn get(&self, key: String) -> Option<Ref<String, Box<dyn Any + Send + Sync>>> {
        self.states.get(&key)
    }

    /// Get a mutable reference to an entry in the map
    ///
    /// **Locking behaviour: May deadlock if called when holding any sort of
    /// reference into the map.** Unfortunately, this is inherited from
    /// [`dashmap`].
    pub fn get_mut(&self, key: String) -> Option<RefMut<String, Box<dyn Any + Send + Sync>>> {
        self.states.get_mut(&key)
    }
}
