use std::marker::PhantomData;

use crate::{BuildedScript, Component, Entity, Scene, Script, ScriptSpecialization};

use super::builder::BuildScript;

pub struct Scheduler {
    groups: Vec<ScriptGroup>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self { groups: Vec::new() }
    }

    pub fn add_group(&mut self, group: ScriptGroup) {
        self.groups.push(group);
    }

    pub fn setup(&mut self, scene: &mut Scene) {
        self.groups.iter_mut().for_each(|group| group.setup(scene));
    }

    pub fn update(&mut self, scene: &mut Scene) {
        self.groups.iter_mut().for_each(|group| group.update(scene));
    }
}

pub struct ScriptGroup {
    script: Box<dyn DynamicScript>,
}

impl ScriptGroup {
    pub fn new<S>(script: impl BuildScript<S>) -> Self
    where
        S: ScriptSpecialization,
    {
        Self {
            script: Box::new(script.build()),
        }
    }

    pub fn init<Script, S>(scene: &mut Scene, base: S) -> Self
    where
        Script: BuildScript<S>,
        S: ScriptSpecialization,
    {
        Self {
            script: Box::new(Script::init(scene, base).build()),
        }
    }

    pub fn setup(&mut self, scene: &mut Scene) {
        self.script.setup(scene);
    }

    pub fn update(&mut self, scene: &mut Scene) {
        self.script.update(scene);
    }
}

pub(crate) trait DynamicScript {
    fn setup(&mut self, scene: &mut Scene);
    fn update(&mut self, scene: &mut Scene);
}

impl<S, T> DynamicScript for BuildedScript<S, T>
where
    S: Script<T>,
    T: ScriptSpecialization,
{
    fn setup(&mut self, scene: &mut Scene) {
        self.setup_script(scene);
    }

    fn update(&mut self, scene: &mut Scene) {
        self.update_script(scene);
    }
}

// Testing
pub trait CustomScheduler<Scrt, Spec, I = ()> {
    //TODO
    fn specialize(&mut self);
    fn insert(&mut self, script: Scrt) {}
    fn run_as_ref(&self, input: I) {}
    fn run(&mut self, input: I) {}
}
