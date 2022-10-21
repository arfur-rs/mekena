//! The market trait of marker traits. A Message is implement for any Send +
//! Sync type.

/// The Message marker trait. Automatically implemented for anything that is
/// Send + Sync.
pub trait Message: Send + Sync {}

impl<T: Send + Sync> Message for T {}
