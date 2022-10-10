//! A node is an element of a [`System`]. It can be composed with other nodes.
//! It can send and recieve messages.

#[async_trait::async_trait]
pub trait Node: Send + Sync {
    async fn starting(&mut self, _ctx: ()) {}
    async fn running(&mut self, _ctx: ()) {}
    async fn stopping(&mut self, _ctx: ()) {}
}
