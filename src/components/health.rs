use specs::{Component, VecStorage};

pub struct HealthComp {
    health: i32,
}

impl Component for HealthComp {
    type Storage = VecStorage<Self>;
}
