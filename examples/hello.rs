use mekena::{context::Context, prelude::*};

/// The main function of the application. Here, we simply register all of our
/// actors and states, and run the system.
///
/// We unwrap to a `miette` error here, but of course, you can unwrap in any way
/// you choose.
#[main]
async fn main(system: System) -> Result<(), miette::Error> {
    system
        .add_node(SomeNode1)
        .add_node(SomeNode2::default())
        //     .add_state(SomeState)
        .start()
        .await?;

    Ok(())
}

/// The structure of `SomeNode1`. Notice that this node does not keep any state.
struct SomeNode1;

#[node]
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

/// The structure of `SomeNode2`. Notice that this node keeps some state. It
/// also derives default, as an easy constructor.
#[derive(Default)]
struct SomeNode2 {
    counter: i32,
}

#[node]
impl Node for SomeNode2 {
    async fn starting(&mut self, _ctx: Context) {
        println!("SomeNode2 starting...");
    }

    /// This will run until the counter reaches 10. Then, it will stop the
    /// *whole* context.
    async fn running(&mut self, ctx: Context) {
        loop {
            if self.counter == 10 {
                ctx.shutdown().await.unwrap();
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
