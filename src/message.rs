pub trait Message: Send + Sync {}

impl<T: Send + Sync> Message for T {}
