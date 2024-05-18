use egeds::*;

fn main() {
    let mut scene = Scene::empty();

    let entity = scene.spawn(());
    let entity_source = scene.get_mut::<Entity>(entity);
    let a_id = entity_source.add(A(10));

    let mut scheduler = Scheduler::new();
    // OR ScriptGroup::new(MyScript::init(&mut scene, entity));
    let group = ScriptGroup::init::<MyScript, Entity>(&mut scene, entity);
    let second_group = ScriptGroup::init::<AnotherScript, Component>(&mut scene, a_id);

    scheduler.add_group(group);
    scheduler.add_group(second_group);
    scheduler.setup(&mut scene);
}

struct A(i32);
struct B(i32);

impl Componentable for A {}
impl Componentable for B {}

pub struct MyScript {
    base: Entity,
}

impl Script<Entity> for MyScript {
    fn init(scene: &mut Scene, base: Entity) -> Self {
        Self { base }
    }
    fn setup(&mut self, scene: &mut Scene) {
        let base = scene.get_mut::<Entity>(self.base);
        let a = base.get_mut::<A>();
        a.0 += 230;
    }
    fn update(&mut self, scene: &mut Scene) {}
}

pub struct AnotherScript {
    base: Component,
}

impl Script<Component> for AnotherScript {
    fn init(scene: &mut Scene, base: Component) -> Self {
        Self { base }
    }

    fn setup(&mut self, scene: &mut Scene) {
        let base = scene.get_ref::<A>(self.base);

        println!("{}", base.0);
    }

    fn update(&mut self, scene: &mut Scene) {}
}
