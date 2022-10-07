use std::marker::PhantomData;

use super::{
    component::Component,
    entity::{Entity, EntityError},
};

pub struct Query<'q, C: Component + 'static> {
    entity: &'q Entity,
    component: PhantomData<C>,
}

pub struct QueryMut<'q, C: Component + 'static> {
    entity: &'q mut Entity,
    component: PhantomData<C>,
}

impl<'q, C: Component + 'static> Query<'q, C> {
    pub fn new(entity: &'q Entity) -> Self {
        Self {
            entity,
            component: PhantomData,
        }
    }

    pub fn query(&self) -> Result<&C, EntityError> {
        self.entity.get::<C>()
    }
}

impl<'q, C: Component + 'static> QueryMut<'q, C> {
    pub fn new(entity: &'q mut Entity) -> Self {
        Self {
            entity,
            component: PhantomData,
        }
    }

    pub fn query_mut(&mut self) -> Result<&mut C, EntityError> {
        self.entity.get_mut::<C>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::world::World;

    #[test]
    fn simple_query() {
        let mut w: World = World::default();
        let e: &mut Entity = w.spawn().unwrap();
        e.insert(0_i32).unwrap();

        Query::<i32>::new(e).query().unwrap();
    }

    #[test]
    fn simple_query_mut() {
        let mut w: World = World::default();
        let e: &mut Entity = w.spawn().unwrap();
        e.insert(0_i32).unwrap();

        let mut _c = QueryMut::<i32>::new(e).query_mut().unwrap();
    }
}
