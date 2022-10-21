//! An example of message selection, for when you have more than one message
//! type that a node should receive.

use mekena::prelude::*;
use tokio::select;

#[main]
async fn main(system: System) -> Result<(), miette::Error> {
    system
        .add_node(SomeNode1)
        .add_node(SomeNode2)
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
    async fn running(&mut self, ctx: &Context) {
        loop {
            select! {
                // Hooray for monomorphization!
                x = ctx.recv::<MyMessage1>() => println!("{x:?}"),
                x = ctx.recv::<MyMessage2>() => println!("{x:?}"),
            };
        }
    }
}

struct SomeNode2;

#[node]
impl Node for SomeNode2 {
    async fn running(&mut self, ctx: &Context) {
        loop {
            ctx.send(MyMessage1).await.unwrap();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}
