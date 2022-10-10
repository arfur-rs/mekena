//! An extension of the "Hello World" application that runs "emergency shutdown"
//! instead of graceful shutdown.

use mekena::prelude::*;

#[mekena::main]
async fn main(system: System) -> Result<(), miette::Error> {
    system
        .add_node(SomeNode1)
        .add_node(SomeNode2::default())
        .start()
        .await?;

    Ok(())
}

struct SomeNode1;

#[async_trait::async_trait]
impl Node for SomeNode1 {
    async fn starting(&mut self, _ctx: Context) {
        println!("SomeNode1 starting...");
    }

    /// This will run indefinitely. Another process will have to kill the ctx in
    /// order for `Self::stopping` to be called.
    async fn running(&mut self, _ctx: Context) {
        loop {
            println!("SomeNode1 running...");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    async fn stopping(&mut self, _ctx: Context) {
        println!("SomeNode1 stopping...");
    }
}

#[derive(Default)]
struct SomeNode2 {
    counter: i32,
}

#[async_trait::async_trait]
impl Node for SomeNode2 {
    async fn starting(&mut self, _ctx: Context) {
        println!("SomeNode2 starting...");
    }

    /// This will run until the counter reaches 10. Then, it will stop the
    /// *whole* context.
    async fn running(&mut self, ctx: Context) {
        loop {
            if self.counter == 10 {
                ctx.emergency_shutdown().await.unwrap();
            } else {
                println!("SomeNode2 running...");
                self.counter += 1;
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }

    async fn stopping(&mut self, _ctx: Context) {
        println!("SomeNode2 stopping...");
    }
}
