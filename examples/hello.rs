use mekena::prelude::*;

/// The main function of the application. Here, we simply register all of our
/// actors and states, and run the system.
///
/// We unwrap to a `miette` error here, but of course, you can unwrap in any way
/// you choose.
#[mekena::main]
fn main(system: System) -> Result<(), miette::Error> {
    // system
    //     .register(SomeActor1)
    //     .register(SomeActor2)
    //     .add_state(SomeState)
    //     .start()?;

    Ok(())
}
