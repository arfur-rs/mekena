pub mod context;
pub mod executor;
pub mod node;
pub mod system;

pub mod prelude {
    pub use crate::context::{Context, ContextError};
    pub use crate::executor::{Executor, ExecutorError, ExecutorState};
    pub use crate::node::Node;
    pub use crate::system::{System, SystemError};
    pub use crate::{main, node};
}

pub use mekena_macros::{main, node};

pub mod re {
    pub use async_trait;
    pub use tokio;
}
