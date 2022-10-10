use crate::node::Node;

pub struct Executor {
    state: ExecutorState,
    nodes: Vec<Box<dyn Node + 'static>>,
}

#[derive(Default)]
pub enum ExecutorState {
    #[default]
    NotStarted,
    Starting,
    Running,
    Stopping,
    Stopped,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            state: ExecutorState::default(),
            nodes: Vec::new(),
        }
    }

    /// Register a node.
    pub fn add_node(&mut self, node: impl Node + 'static) {
        self.nodes.push(Box::new(node));
    }

    pub async fn start(mut self) -> Result<(), ExecutorError> {
        self.state = ExecutorState::Starting;
        let started = Self::starting(self.nodes).await?;

        self.state = ExecutorState::Running;
        let ran = Self::running(started).await?;

        self.state = ExecutorState::Stopping;
        let _stopped = Self::stopping(ran).await?;

        self.state = ExecutorState::Stopped;

        Ok(())
    }

    /// Run the [`Node::starting`] method on every node.
    async fn starting(nodes: Vec<Box<dyn Node>>) -> Result<Vec<Box<dyn Node>>, ExecutorError> {
        let mut handles = Vec::with_capacity(nodes.len());

        for mut node in nodes {
            let handle = async move {
                node.starting(()).await;
                node
            };

            handles.push(handle);
        }

        let output = futures::future::try_join_all(handles.into_iter().map(tokio::spawn))
            .await
            .unwrap();

        Ok(output)
    }

    /// Run the [`Node::starting`] method on every node.
    async fn running(nodes: Vec<Box<dyn Node>>) -> Result<Vec<Box<dyn Node>>, ExecutorError> {
        let mut handles = Vec::with_capacity(nodes.len());

        for mut node in nodes {
            let handle = async move {
                node.running(()).await;
                node
            };

            handles.push(handle);
        }

        let output = futures::future::try_join_all(handles.into_iter().map(tokio::spawn))
            .await
            .unwrap();

        Ok(output)
    }

    /// Run the [`Node::starting`] method on every node.
    async fn stopping(nodes: Vec<Box<dyn Node>>) -> Result<Vec<Box<dyn Node>>, ExecutorError> {
        let mut handles = Vec::with_capacity(nodes.len());

        for mut node in nodes {
            let handle = async move {
                node.stopping(()).await;
                node
            };

            handles.push(handle);
        }

        let output = futures::future::try_join_all(handles.into_iter().map(tokio::spawn))
            .await
            .unwrap();

        Ok(output)
    }
}

#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum ExecutorError {
    #[error("An unknown error occured.")]
    #[diagnostic(code(mekena::context::unknown))]
    Unknown,
}
