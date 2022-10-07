use crate::ecs::world::World;

pub struct System {
    /// Stores worldwide state.
    world: World,
}

impl System {
    pub fn start(&mut self) {}
}

impl Default for System {
    fn default() -> Self {
        Self {
            world: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds() {
        System::default();
    }

    #[test]
    fn starts() {
        System::default().start();
    }
}
