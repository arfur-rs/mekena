#![feature(result_flattening)]

pub mod ecs;
pub mod subsystem;
pub mod system;

pub mod prelude {
    pub use super::system::System;
}
