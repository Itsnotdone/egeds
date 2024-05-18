use std::any::Any;

use crate::{EntityId, Id};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Component {
    pub id: ComponentId,
}

impl Id for Component {
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
pub struct ComponentId {
    pub parent: ParentId,
    pub own: usize,
}

impl Id for ComponentId {
    fn is(&self, id: impl Id) -> bool {
        id.value() == self.own && id.parent() == self.parent()
    }

    fn value(&self) -> usize {
        self.own
    }

    fn component(&self) -> Option<ComponentId> {
        Some(*self)
    }

    fn parent(&self) -> ParentId {
        self.parent
    }
}

pub struct ComponentSource {
    pub id: ComponentId,
    pub(crate) value: Box<dyn Componentable>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParentId {
    Entity(EntityId),
    Scene,
}

pub trait Componentable: Any {}

pub type ComponentArray = Vec<ComponentSource>;
