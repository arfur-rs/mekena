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
    async fn starting(&mut self, _ctx: &Context, _mb: &Mailbox) {
        println!("SomeNode1 starting...");
    }

    async fn running(&mut self, _ctx: &Context, mb: &Mailbox) {
        loop {
            println!("SomeNode1 running...");
            let m: Option<Box<MyMessage>> = mb.recv().await.unwrap();
            println!("Received a message: {m:?}");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    async fn stopping(&mut self, _ctx: &Context, _mb: &Mailbox) {
        println!("SomeNode1 stopping...");
    }
}

#[derive(Default)]
struct SomeNode2 {
    counter: i32,
}

#[node]
impl Node for SomeNode2 {
    async fn starting(&mut self, _ctx: &Context, _mb: &Mailbox) {
        println!("SomeNode2 starting...");
    }

    async fn running(&mut self, ctx: &Context, mb: &Mailbox) {
        loop {
            if self.counter == 10 {
                // If we're at 10, stop.
                ctx.shutdown().await.unwrap();
            } else if (self.counter % 2) == 0 {
                // If we're at an even number, send a message.
                mb.send(MyMessage(self.counter)).await.unwrap();
            }

            println!("SomeNode2 running...");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            self.counter += 1;
        }
    }

    async fn stopping(&mut self, _ctx: &Context, _mb: &Mailbox) {
        println!("SomeNode2 stopping...");
    }
}
