use std::time::Instant;

use parry2d::na::Vector2;
use specs::Component;
use specs::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Physics {
    pub world_space_position: Vector2<f32>,

    /// directional vector of the object
    /// denoting "forward"
    pub direction: Vector2<f32>,

    /// speed of the object
    /// functionally a scalar of the direction vector
    pub speed: f32,

    pub mass: f32,

    // system needed info
    pub last_time_updated: Instant,
}

impl Component for Physics {
    type Storage = VecStorage<Self>;
}
