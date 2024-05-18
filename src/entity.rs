use std::any::Any;

use crate::{Component, ComponentArray, ComponentId, ComponentSource, Componentable, Id, ParentId};

#[derive(Debug, Clone, Copy)]
pub struct Entity {
    pub id: EntityId,
}

impl Id for Entity {
    fn is(&self, id: impl Id) -> bool {
        self.id.is(id)
    }

    fn value(&self) -> usize {
        self.id.value()
    }

    fn entity(&self) -> Option<EntityId> {
        self.id.entity()
    }

    fn parent(&self) -> ParentId {
        self.id.parent()
    }

    fn component(&self) -> Option<ComponentId> {
        self.id.component()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityId(pub usize);

impl Id for EntityId {
    fn is(&self, id: impl Id) -> bool {
        id.value() == self.0
    }

    fn value(&self) -> usize {
        self.0
    }

    fn entity(&self) -> Option<EntityId> {
        Some(*self)
    }

    fn parent(&self) -> ParentId {
        ParentId::Scene
    }
}

pub struct EntitySource {
    pub id: EntityId,
    pub(crate) components: ComponentArray,
}

impl EntitySource {
    pub fn empty(id: EntityId) -> EntitySource {
        EntitySource {
            id,
            components: ComponentArray::new(),
        }
    }

    pub fn add(&mut self, component: impl Componentable) -> Component {
        let id = ComponentId {
            parent: ParentId::Entity(self.id),
            own: self.components.len(),
        };
        let component_source = ComponentSource {
            id: id,
            value: Box::new(component),
        };

        self.components.push(component_source);

        Component { id: id }
    }

    pub fn get<'a, T>(&'a mut self) -> T::Item<'a>
    where
        T: EntityFetch,
    {
        self.try_get::<T>().unwrap()
    }

    pub fn try_get<'a, T>(&'a mut self) -> Option<T::Item<'a>>
    where
        T: EntityFetch,
    {
        T::fetch(self)
    }

    pub fn get_ref<T>(&self) -> &T
    where
        T: Componentable,
    {
        self.try_get_ref().unwrap()
    }

    pub fn get_mut<T>(&mut self) -> &mut T
    where
        T: Componentable,
    {
        self.try_get_mut().unwrap()
    }

    pub fn try_get_ref<T>(&self) -> Option<&T>
    where
        T: Componentable,
    {
        self.components
            .iter()
            .find_map(|component| (component.value.as_ref() as &dyn Any).downcast_ref::<T>())
    }

    pub fn try_get_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Componentable,
    {
        self.components
            .iter_mut()
            .find_map(|component| (component.value.as_mut() as &mut dyn Any).downcast_mut::<T>())
    }
}

pub trait EntityFetch {
    type Item<'a>;

    fn fetch<'a>(entity: &'a mut EntitySource) -> Option<Self::Item<'a>>;
}

impl<T> EntityFetch for &T
where
    T: Componentable,
{
    type Item<'a> = &'a T;

    fn fetch<'a>(entity: &'a mut EntitySource) -> Option<Self::Item<'a>> {
        entity.try_get_ref::<T>()
    }
}

impl<T> EntityFetch for &mut T
where
    T: Componentable,
{
    type Item<'a> = &'a mut T;

    fn fetch<'a>(entity: &'a mut EntitySource) -> Option<Self::Item<'a>> {
        entity.try_get_mut::<T>()
    }
}

pub type EntityArray = Vec<EntitySource>;
