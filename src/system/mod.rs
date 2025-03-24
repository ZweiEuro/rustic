use crate::gen_vec::GenVec;

pub enum SystemType {
    RunOnce,
    Continuous(i32 /* per second */),
}

pub struct System<T>
where
    T: Send + Sync,
{
    pub system_type: SystemType,
    pub running: bool,
    pub components: GenVec<Option<T>>,
}

impl<T> System<T>
where
    T: Send + Sync,
{
    pub fn new(sys_type: SystemType, entities: &mut GenVec<Option<T>>) -> System<T> {
        System {
            system_type: sys_type,
            running: true,
            components: entities,
        }
    }
}
