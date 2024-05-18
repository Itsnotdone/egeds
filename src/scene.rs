use crate::{
    Bundle, Component, ComponentId, ComponentSource, Componentable, Entity, EntityArray, EntityId,
    EntitySource, ParentId,
};

pub struct Scene {
    pub(crate) entities: EntityArray,
    pub(crate) next_id: usize,
}

impl Scene {
    pub fn empty() -> Scene {
        Scene {
            entities: EntityArray::new(),
            next_id: 0,
        }
    }

    pub fn spawn(&mut self, bundle: impl Bundle) -> Entity {
        bundle.spawn(self)
    }

    pub fn add_entity(&mut self, mut entity: EntitySource) -> Entity {
        let id = EntityId(self.next_id);

        entity.id = id;
        self.entities.push(entity);
        self.next_id += 1;

        Entity { id: id }
    }

    pub fn get_ref<T>(&self, id: impl Id) -> &T::Item
    where
        T: SceneFetch,
    {
        self.try_get_ref::<T>(id).unwrap()
    }

    pub fn get_mut<T>(&mut self, id: impl Id) -> &mut T::Item
    where
        T: SceneFetch,
    {
        self.try_get_mut::<T>(id).unwrap()
    }

    pub fn try_get_ref<T>(&self, id: impl Id) -> Option<&T::Item>
    where
        T: SceneFetch,
    {
        T::fetch_ref(id, &self.entities)
    }

    pub fn try_get_mut<T>(&mut self, id: impl Id) -> Option<&mut T::Item>
    where
        T: SceneFetch,
    {
        T::fetch_mut(id, &mut self.entities)
    }
}

pub trait SceneFetch {
    type Item;

    fn fetch_ref<'a>(id: impl Id, entities: &'a EntityArray) -> Option<&'a Self::Item>;
    fn fetch_mut<'a>(id: impl Id, entities: &'a mut EntityArray) -> Option<&'a mut Self::Item>;
}

impl SceneFetch for Entity {
    type Item = EntitySource;

    fn fetch_ref<'a>(id: impl Id, entities: &'a EntityArray) -> Option<&'a Self::Item> {
        let id = id.entity()?;

        entities.get(id.0)
    }

    fn fetch_mut<'a>(id: impl Id, entities: &'a mut EntityArray) -> Option<&'a mut Self::Item> {
        let id = id.entity()?;

        entities.get_mut(id.0)
    }
}

impl<T: Componentable> SceneFetch for T {
    type Item = T;

    fn fetch_ref<'a>(id: impl Id, entities: &'a EntityArray) -> Option<&'a Self::Item> {
        let id = id.component()?;

        match id.parent() {
            ParentId::Entity(eid) => {
                let entity = entities.get(eid.0)?;

                entity.try_get_ref::<T>()
            }
            _ => None,
        }
    }

    fn fetch_mut<'a>(id: impl Id, entities: &'a mut EntityArray) -> Option<&'a mut Self::Item> {
        let id = id.component()?;

        match id.parent() {
            ParentId::Entity(eid) => {
                let entity = entities.get_mut(eid.0)?;

                entity.try_get_mut::<T>()
            }
            _ => None,
        }
    }
}

pub trait Id: Copy {
    fn is(&self, id: impl Id) -> bool;
    fn value(&self) -> usize;
    fn parent(&self) -> ParentId;
    fn component(&self) -> Option<ComponentId> {
        None
    }
    fn entity(&self) -> Option<EntityId> {
        None
    }
}
