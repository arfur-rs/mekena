use tokio::select;

use crate::{context::Context, node::Node};

pub struct Executor {
    state: ExecutorState,
    nodes: Vec<Box<dyn Node + 'static>>,
    context: Context,
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
    pub fn new(context: Context) -> Self {
        Self {
            state: ExecutorState::default(),
            nodes: Vec::new(),
            context,
        }
    }

    /// Register a node.
    pub fn add_node(&mut self, node: impl Node + 'static) {
        self.nodes.push(Box::new(node));
    }

    pub async fn start(mut self) -> Result<(), ExecutorError> {
        self.state = ExecutorState::Starting;
        let started = Self::starting(self.nodes, self.context.clone()).await?;

        self.state = ExecutorState::Running;
        let ran = Self::running(started, self.context.clone()).await?;

        self.state = ExecutorState::Stopping;
        let _stopped = Self::stopping(ran, self.context.clone()).await?;

        self.state = ExecutorState::Stopped;

        Ok(())
    }

    /// Run the [`Node::starting`] method on every node.
    async fn starting(
        nodes: Vec<Box<dyn Node>>,
        context: Context,
    ) -> Result<Vec<Box<dyn Node>>, ExecutorError> {
        let mut handles = Vec::with_capacity(nodes.len());

        for mut node in nodes {
            let context = context.clone();
            let handle = async move {
                node.starting(context).await;
                node
            };

            handles.push(handle);
        }

        let output = futures::future::join_all(handles);
        select! {
            x = output => Ok(x),
            _ = context.wait_for_shutdown() => Ok(Vec::new()),
            _ = context.wait_for_emergency_shutdown() => Err(ExecutorError::Shutdown),
        }
    }

    /// Run the [`Node::starting`] method on every node.
    async fn running(
        nodes: Vec<Box<dyn Node>>,
        context: Context,
    ) -> Result<Vec<Box<dyn Node>>, ExecutorError> {
        let mut handles = Vec::with_capacity(nodes.len());

        for mut node in nodes {
            let context = context.clone();

            let handle = async move {
                node.running(context).await;
                node
            };

            handles.push(handle);
        }

        let output = futures::future::join_all(handles);
        select! {
            x = output => Ok(x),
            _ = context.wait_for_shutdown() => Ok(Vec::new()),
            _ = context.wait_for_emergency_shutdown() => Err(ExecutorError::Shutdown),
        }
    }

    /// Run the [`Node::starting`] method on every node.
    async fn stopping(
        nodes: Vec<Box<dyn Node>>,
        context: Context,
    ) -> Result<Vec<Box<dyn Node>>, ExecutorError> {
        let mut handles = Vec::with_capacity(nodes.len());

        for mut node in nodes {
            let context = context.clone();
            let handle = async move {
                node.stopping(context).await;
                node
            };

            handles.push(handle);
        }

        let output = futures::future::join_all(handles);
        select! {
            x = output => Ok(x),
            _ = context.wait_for_shutdown() => Ok(Vec::new()),
            _ = context.wait_for_emergency_shutdown() => Err(ExecutorError::Shutdown),
        }
    }
}

#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum ExecutorError {
    #[error("The context was commanded to shut down.")]
    #[diagnostic(code(mekena::executor::shutdown))]
    Shutdown,

    #[error("An unknown error occured.")]
    #[diagnostic(code(mekena::executor::unknown))]
    Unknown,
}
