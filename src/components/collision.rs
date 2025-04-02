use parry2d::shape::Shape;
use specs::prelude::*;

pub struct CollisionComp {
    pub collision_shape: Box<dyn Shape>,
}

impl Component for CollisionComp {
    type Storage = VecStorage<Self>;
}
