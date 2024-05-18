use std::marker::PhantomData;

use crate::{Component, Entity, Scene};

pub trait Script<S>: 'static {
    fn init(scene: &mut Scene, base: S) -> Self;
    fn setup(&mut self, scene: &mut Scene) {}
    fn update(&mut self, scene: &mut Scene) {}
}

pub trait ScriptSpecialization: 'static {}

impl ScriptSpecialization for Entity {}
impl ScriptSpecialization for Component {}

pub struct BuildedScript<Scrt, Spec> {
    pub(crate) script: Scrt,
    pub(crate) specialization: PhantomData<Spec>,
}

impl<Scrt, Spec> BuildedScript<Scrt, Spec>
where
    Scrt: Script<Spec>,
    Spec: ScriptSpecialization,
{
    pub(crate) fn setup_script(&mut self, scene: &mut Scene) {
        self.script.setup(scene);
    }
    pub(crate) fn update_script(&mut self, scene: &mut Scene) {
        self.script.update(scene);
    }
}
