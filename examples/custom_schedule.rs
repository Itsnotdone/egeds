use egeds::*;

fn main() {}

pub struct PreUpdateSchedule {
    scripts: Vec<ScriptGroup>,
}

impl<Scrt, Spec> CustomScheduler<Scrt, Spec, &mut Scene> for PreUpdateSchedule
where
    Scrt: Script<Spec>,
    Spec: ScriptSpecialization,
{
    fn specialize(&mut self) {}
    fn insert(&mut self, script: Scrt) {
        self.scripts.push(ScriptGroup::new(script));
    }

    fn run(&mut self, input: &mut Scene) {
        self.scripts.iter_mut().for_each(|script| {})
    }

    fn run_as_ref(&self, input: &mut Scene) {}
}
