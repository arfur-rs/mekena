//! A node is an element of a [`System`]. It can be composed with other nodes.
//! It can send and recieve messages.

use crate::context::Context;

#[async_trait::async_trait(?Send)]
pub trait Node {
    async fn starting(&mut self, _ctx: &Context) {}
    async fn running(&mut self, _ctx: &Context) {}
    async fn stopping(&mut self, _ctx: &Context) {}
}
