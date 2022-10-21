use flume::{Receiver, Sender};

pub struct ShutdownManager {
    sender: Sender<()>,
    receiver: Receiver<()>,
}

impl ShutdownManager {
    pub fn new() -> Self {
        let (sender, receiver) = flume::bounded(1);

        Self { sender, receiver }
    }

    pub async fn shutdown(&self) {
        self.sender.send_async(()).await.unwrap()
    }

    pub async fn await_shutdown(&self) {
        self.receiver.recv_async().await.unwrap()
    }
}
