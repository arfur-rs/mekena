//! An example of message selection, for when you have more than one message
//! type that a node should receive.

use mekena::prelude::*;
use tokio::select;

#[main]
async fn main(system: System) -> Result<(), miette::Error> {
    system
        .add_node(SomeNode1)
        .add_node(SomeNode2::default())
        .start()
        .await?;

    Ok(())
}

#[derive(Debug)]
struct MyMessage1;

#[derive(Debug)]
struct MyMessage2;

struct SomeNode1;

#[node]
impl Node for SomeNode1 {
    async fn running(&mut self, _ctx: &Context, mb: &Mailbox) {
        loop {
            select! {
                // Hooray for monomorphization!
                x = mb.recv::<MyMessage1>() => println!("{x:?}"),
                x = mb.recv::<MyMessage2>() => println!("{x:?}"),
            };
        }
    }
}

#[derive(Default)]
struct SomeNode2;

#[node]
impl Node for SomeNode2 {
    async fn running(&mut self, _: &Context, mb: &Mailbox) {
        loop {
            mb.send(MyMessage1).await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}
