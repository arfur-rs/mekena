use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::Arc,
};

use thiserror::Error;

use super::component::Component;

#[derive(Error, Debug)]
pub enum EntityError {
    #[error("could not find component")]
    ComponentNotFound,
    #[error("could not downcast internally represented type")]
    CouldNotDowncast,
    #[error("could not make internal type mutable")]
    CouldNotMakeMutable,
    #[error("component already exists")]
    ComponentAlreadyExists,
}

pub struct Entity {
    id: usize,
    components: HashMap<TypeId, Arc<dyn Any>>,
}

impl Entity {
    /// Create a new entity based on a given ID.
    pub fn new(id: usize) -> Self {
        Self {
            id,
            components: HashMap::<TypeId, Arc<dyn Any>>::new(),
        }
    }

    /// Get this entity's ID.
    pub fn id(&self) -> usize {
        self.id
    }

    /// Insert a component into the ComponentVec.
    pub fn insert<C: Component + 'static>(&mut self, c: C) -> Result<(), EntityError> {
        let id = TypeId::of::<C>();

        if self.components.contains_key(&id) {
            Err(EntityError::ComponentAlreadyExists)
        } else {
            self.components.insert(id, Arc::new(c));
            Ok(())
        }
    }

    /// Get a component from the ComponentVec.
    pub fn get<C: Component + 'static>(&self) -> Result<&C, EntityError> {
        self.components
            .get(&TypeId::of::<C>())
            .ok_or(EntityError::ComponentNotFound)
            .map(|c| c.downcast_ref::<C>())
            .map(|t| t.ok_or(EntityError::CouldNotDowncast))
            .flatten()
    }

    /// Get a component from the ComponentVec.
    pub fn get_mut<C: Component + 'static>(&mut self) -> Result<&mut C, EntityError> {
        self.components
            .get_mut(&TypeId::of::<C>())
            .ok_or(EntityError::ComponentNotFound)
            .map(|c| Arc::get_mut(c))
            .map(|c| c.ok_or(EntityError::CouldNotMakeMutable))
            .flatten()
            .map(|c| c.downcast_mut::<C>())
            .map(|t| t.ok_or(EntityError::CouldNotDowncast))
            .flatten()
    }
}
#[cfg(test)]
mod tests {

    use crate::ecs::world::World;

    #[derive(Debug, PartialEq, Eq)]
    struct MyComponent(i32);

    #[test]
    fn id() {
        let mut world = World::default();
        let entity = world.spawn().unwrap();
        assert_eq!(entity.id(), 0);
    }

    #[test]
    fn insert_components() {
        let mut world = World::default();

        let entity = world.spawn().unwrap();

        entity.insert(0_i32).unwrap();
        entity.insert(0_u64).unwrap();
        entity.insert(0_usize).unwrap();
    }

    #[test]
    fn insert_nonstd_components() {
        let mut world = World::default();

        let entity = world.spawn().unwrap();
        entity.insert(MyComponent(0)).unwrap();
    }

    #[test]
    #[should_panic]
    fn insert_same_components() {
        let mut world = World::default();

        let entity = world.spawn().unwrap();
        entity.insert(0_i32).unwrap();
        entity.insert(0_i32).unwrap();
    }

    #[test]
    fn get() {
        let component = MyComponent(0);
        let mut world = World::default();
        let entity = world.spawn().unwrap();
        entity.insert(component).unwrap();
    }
}
