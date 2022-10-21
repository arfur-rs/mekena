//! Let's experiment with how accurate the timing is of `mekena-messaging`.

use lazy_static::lazy_static;
use mekena::prelude::*;
use tokio::time::Instant;

lazy_static! {
    static ref START_TIME: Instant = tokio::time::Instant::now();
}

#[main]
async fn main(system: System) -> Result<(), miette::Error> {
    system.add_node(SomeNode1).start().await?;

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
    async fn running(&mut self, _ctx: &Context) {
        loop {
            let elapsed = START_TIME.elapsed();
            println!("{elapsed:?}");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}
