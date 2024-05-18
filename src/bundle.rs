use crate::{Componentable, Entity, EntityId, EntitySource, Scene};

pub trait Bundle {
    fn insert(self, entity: &mut EntitySource);
    fn spawn(self, scene: &mut Scene) -> Entity;
}

impl Bundle for () {
    fn insert(self, entity: &mut EntitySource) {}

    fn spawn(self, scene: &mut Scene) -> Entity {
        scene.add_entity(EntitySource::empty(EntityId(0)))
    }
}

impl<T> Bundle for T
where
    T: Componentable,
{
    fn insert(self, entity: &mut EntitySource) {
        entity.add(self);
    }

    fn spawn(self, scene: &mut Scene) -> Entity {
        let mut entity = EntitySource::empty(EntityId(0));
        entity.add(self);
        scene.add_entity(entity)
    }
}

macro_rules! impl_for_tuples {
    ($($Generic:ident, $Index:tt),*) => {
        impl<$($Generic,)*> Bundle for ($($Generic,)*)
        where
            $($Generic: Bundle,)*
        {
            fn insert(self, entity: &mut EntitySource) {
                $(self.$Index.insert(entity);)*
            }

            fn spawn(self, scene: &mut Scene) -> Entity {
                let mut entity = EntitySource::empty(EntityId(0));
                $(self.$Index.insert(&mut entity);)*
                scene.add_entity(entity)
            }
        }
    };
}

impl_for_tuples!(T0, 0, T1, 1);
impl_for_tuples!(T0, 0, T1, 1, T2, 2);
impl_for_tuples!(T0, 0, T1, 1, T2, 2, T3, 3);
impl_for_tuples!(T0, 0, T1, 1, T2, 2, T3, 3, T4, 4);
impl_for_tuples!(T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5);
impl_for_tuples!(T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6);
impl_for_tuples!(T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7);
impl_for_tuples!(T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8);
impl_for_tuples!(T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9);
impl_for_tuples!(T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10);
impl_for_tuples!(
    T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10, T11, 11
);
impl_for_tuples!(
    T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10, T11, 11, T12, 12
);
impl_for_tuples!(
    T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10, T11, 11, T12,
    12, T13, 13
);
impl_for_tuples!(
    T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10, T11, 11, T12,
    12, T13, 13, T14, 14
);
impl_for_tuples!(
    T0, 0, T1, 1, T2, 2, T3, 3, T4, 4, T5, 5, T6, 6, T7, 7, T8, 8, T9, 9, T10, 10, T11, 11, T12,
    12, T13, 13, T14, 14, T15, 15
);
