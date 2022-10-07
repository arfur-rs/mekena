pub trait Component {}

impl<T: Send + Sync + 'static> Component for T {}
