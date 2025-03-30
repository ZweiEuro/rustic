use parry2d::shape::Shape;
use specs::prelude::*;

pub struct Collision {
    pub collision_shape: Box<dyn Shape>,
}

impl Component for Collision {
    type Storage = VecStorage<Self>;
}
