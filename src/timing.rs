//! Let's experiment with how accurate the timing is of `mekena-messaging`.

use mekena::prelude::*;

#[main]
async fn main(system: System) -> Result<(), miette::Error> {
    system
        .add_node(SomeNode1)
        .add_node(SomeNode2::default())
        .start()
        .await?;

    Ok(())
}

/// Any type that is Send + Sync implements the Message marker trait by default.
/// We also derive(Debug) here for debugging purposes.
#[derive(Debug)]
struct MyMessage(i32);

struct SomeNode1;

/// A node that will loop {} listening for a message of a specific type.
#[node]
impl Node for SomeNode1 {
    async fn running(&mut self, ctx: &Context) {
        loop {
            let m: Box<MyMessage> = ctx.recv().await.unwrap();
            println!("Received a message: {m:?}");
        }
    }
}

#[derive(Default)]
struct SomeNode2 {
    counter: i32,
}

#[node]
impl Node for SomeNode2 {
    async fn running(&mut self, ctx: &Context) {
        loop {
            if self.counter == 10 {
                // If we're at 10, stop.
                ctx.shutdown().await;
            } else if (self.counter % 2) == 0 {
                // If we're at an even number, send a message.
                ctx.send(MyMessage(self.counter)).await.unwrap();
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            self.counter += 1;
        }
    }
}
