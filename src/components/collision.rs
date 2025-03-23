use parry2d::shape::Shape;

pub struct Collision {
    collision_shape: Box<dyn Shape>,
}
