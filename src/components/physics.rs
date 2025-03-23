use parry2d::na::Vector2;

pub struct Physics {
    world_space_position: Vector2<f32>,
    velocity: Vector2<f32>,
    mass: f32,
}
