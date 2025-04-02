use core::panic;
use std::time::Instant;

use parry2d::na::Vector2;
use sdl3::pixels::Color;
use specs::Component;
use specs::prelude::*;

use super::CollisionComp;
use super::DrawableComp;

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Rectangle { width: f32, height: f32 },
    Circle { radius: f32 },
}

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

    pub shape: Shape,
}

impl Into<CollisionComp> for Physics {
    fn into(self) -> CollisionComp {
        CollisionComp {
            collision_shape: match self.shape {
                Shape::Rectangle { width, height } => Box::new(parry2d::shape::Cuboid::new(
                    Vector2::new(width / 2.0, height / 2.0),
                )),

                Shape::Circle { radius } => Box::new(parry2d::shape::Ball::new(radius)),

                _ => panic!(),
            },
        }
    }
}

impl Into<DrawableComp> for Physics {
    fn into(self) -> DrawableComp {
        DrawableComp {
            shape: self.shape.clone(),
            color: Color::RGB(255, 0, 0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PhysicsComp {
    pub physics: Physics,

    // system needed info
    pub last_time_updated: Instant,
}

impl PhysicsComp {
    pub fn new(ph: Physics) -> Self {
        PhysicsComp {
            physics: ph,
            last_time_updated: Instant::now(),
        }
    }
}

impl Component for PhysicsComp {
    type Storage = VecStorage<Self>;
}
