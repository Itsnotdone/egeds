use std::marker::PhantomData;

use crate::{Component, Entity, ScriptSpecialization};

use super::script::{BuildedScript, Script};

pub trait BuildScript<S>: Script<S> + 'static {
    fn build(self) -> BuildedScript<Self, S>
    where
        Self: Sized;
}

impl<T, Spec> BuildScript<Spec> for T
where
    T: Script<Spec>,
    Spec: ScriptSpecialization,
{
    fn build(self) -> BuildedScript<T, Spec> {
        BuildedScript {
            script: self,
            specialization: PhantomData,
        }
    }
}
