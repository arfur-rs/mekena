use thiserror::Error;

use super::entity::Entity;

#[derive(Error, Debug)]
pub enum WorldError {
    #[error("resource already exists")]
    ResourceAlreadyExists,
}

#[derive(Default)]
pub struct World {
    entities: Vec<Entity>,
}

impl World {
    /// Create a new entity, add it to the World, and return it.
    pub fn spawn(&mut self) -> Result<&mut Entity, WorldError> {
        let e = Entity::new(self.entities.len());
        self.entities.push(e);
        self.entities
            .last_mut()
            .ok_or(WorldError::ResourceAlreadyExists)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construction() {
        World::default();
    }

    #[test]
    fn spawn_entity() {
        let mut world = World::default();
        world.spawn().unwrap();
    }

    #[test]
    fn spawn_entities() {
        let mut world = World::default();

        let _entity_a = world.spawn().unwrap();
        let _entity_b = world.spawn().unwrap();
        let _entity_c = world.spawn().unwrap();
    }
}
