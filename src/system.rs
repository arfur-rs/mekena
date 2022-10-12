use tokio::select;

use crate::{context::Context, mailbox::Mailbox, node::Node};

pub struct System {
    state: SystemState,
    nodes: Vec<Box<dyn Node + 'static>>, // TODO: can we figure this out at compile time?
    context: Context,
    mailbox: Mailbox,
}

#[derive(Copy, Clone, Debug, Default)]
pub enum SystemState {
    #[default]
    NotStarted,
    Starting,
    Running,
    Stopping,
}

pub enum NextState {
    Continue,
    Stop,
}

impl System {
    pub fn new() -> Self {
        Self {
            state: SystemState::default(),
            nodes: Vec::new(),
            context: Context::new(),
            mailbox: Mailbox::new(),
        }
    }

    /// Register a node.
    pub fn add_node(mut self, node: impl Node + 'static) -> Self {
        self.nodes.push(Box::new(node));
        self
    }

    pub async fn start(&mut self) -> Result<(), SystemError> {
        match self.starting().await? {
            NextState::Continue => match self.running().await? {
                _ => self.stopping().await?,
            },
            NextState::Stop => self.stopping().await?,
        };

        Ok(())
    }

    async fn starting(&mut self) -> Result<NextState, SystemError> {
        self.state = SystemState::Starting;
        let mut handles = Vec::with_capacity(self.nodes.len());

        for node in &mut self.nodes {
            let handle = async {
                node.starting(&self.context, &self.mailbox).await;
            };

            handles.push(handle);
        }

        let output = futures::future::join_all(handles);

        select! {
            _ = output => Ok(NextState::Continue),
            _ = self.context.wait_for_shutdown() => Ok(NextState::Stop),
            _ = self.context.wait_for_emergency_shutdown() => Err(SystemError::Shutdown),
        }
    }

    async fn running(&mut self) -> Result<NextState, SystemError> {
        self.state = SystemState::Running;
        let mut handles = Vec::with_capacity(self.nodes.len());

        for node in &mut self.nodes {
            let handle = async {
                node.running(&self.context, &self.mailbox).await;
            };

            handles.push(handle);
        }

        let output = futures::future::join_all(handles);

        select! {
            _ = output => Ok(NextState::Continue),
            _ = self.context.wait_for_shutdown() => Ok(NextState::Stop),
            _ = self.context.wait_for_emergency_shutdown() => Err(SystemError::Shutdown),
        }
    }

    async fn stopping(&mut self) -> Result<NextState, SystemError> {
        self.state = SystemState::Stopping;
        let mut handles = Vec::with_capacity(self.nodes.len());

        for node in &mut self.nodes {
            let handle = async {
                node.stopping(&self.context, &self.mailbox).await;
            };

            handles.push(handle);
        }

        let output = futures::future::join_all(handles);

        select! {
            _ = output => Ok(NextState::Continue),
            _ = self.context.wait_for_shutdown() => Ok(NextState::Stop),
            _ = self.context.wait_for_emergency_shutdown() => Err(SystemError::Shutdown),
        }
    }

    pub fn get_state(&self) -> SystemState {
        self.state
    }
}

#[derive(thiserror::Error, miette::Diagnostic, Debug)]
pub enum SystemError {
    #[error("The context was commanded to shut down.")]
    #[diagnostic(code(mekena::system::shutdown))]
    Shutdown,

    #[error("An unknown error occured.")]
    #[diagnostic(code(mekena::system::unknown))]
    Unknown,
}
