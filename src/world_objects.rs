use std::time::Instant;

use parry2d::na::Vector2;
use sdl3::pixels::Color;
use specs::prelude::*;

use crate::components::{Collision, Drawable, DrawableType, Physics};

pub fn create_rect(
    world: &mut World,
    position: [f32; 2],
    dimensions: [f32; 2],
    velocity: Option<[f32; 2]>,
    mass: Option<f32>,
    color: Option<Color>,
) -> Entity {
    world
        .create_entity()
        .with(Physics {
            world_space_position: Vector2::new(position[0], position[1]),
            velocity: velocity.map_or(Vector2::new(0.0, 0.0), |v| Vector2::new(v[0], v[1])),
            mass: mass.unwrap_or(1.0),
            last_time_updated: Instant::now(),
        })
        .with(Drawable {
            drawable_type: DrawableType::Rectangle,
            color: color.unwrap_or(Color::RGB(255, 0, 0)),
            width: dimensions[0],
            height: dimensions[1],
            radius: 0.0,
        })
        .with(Collision {
            collision_shape: Box::new(parry2d::shape::Cuboid::new(Vector2::new(
                dimensions[0] / 2.0,
                dimensions[1] / 2.0,
            ))),
        })
        .build()
}
