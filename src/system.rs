use crate::{
    executor::{Executor, ExecutorError},
    node::Node,
};

/// A system. The father of the framework.
pub struct System {
    executor: Executor,
}

impl System {
    pub fn new() -> Self {
        Self {
            executor: Executor::new(),
        }
    }

    /// Register a node.
    pub fn add_node(mut self, node: impl Node + 'static) -> Self {
        self.executor.add_node(node);
        self
    }

    pub async fn start(self) -> Result<(), SystemError> {
        self.executor.start().await?;
        Ok(())
    }
}

#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum SystemError {
    #[error(transparent)]
    ExecutorError(#[from] ExecutorError),

    #[error("An unknown error occured.")]
    #[diagnostic(code(mekena::system::unknown))]
    Unknown,
}

#[cfg(test)]
mod tests {
    use tokio::test;

    use super::*;

    #[test]
    async fn builds() {
        System::new();
    }

    #[test]
    async fn starts() {
        System::new().start().await.unwrap();
    }
}
