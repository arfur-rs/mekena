//! An example of using the emergency shutdown functionality, instead of the
//! graceful one.

use mekena::prelude::*;

#[main]
async fn main(system: System) -> Result<(), miette::Error> {
    system.add_node(SomeNode1::default()).start().await?;

    Ok(())
}

#[derive(Default)]
struct SomeNode1 {
    counter: i32,
}

#[node]
impl Node for SomeNode1 {
    /// This will be run three times, then it will quit.
    async fn running(&mut self, ctx: &Context, _: &Mailbox) {
        loop {
            if self.counter == 3 {
                ctx.emergency_shutdown().await.unwrap();
            } else {
                println!("SomeNode1 running...");
                self.counter += 1;
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }

    /// This will never be run, as opposed to ctx.shutdown() which will try to
    /// gracefully shut down by running Node::stopping.
    async fn stopping(&mut self, _: &Context, _: &Mailbox) {
        println!("This is never run!");
    }
}
