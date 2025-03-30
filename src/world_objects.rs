use std::{any::Any, time::Instant};

use parry2d::na::Vector2;
use sdl3::{event::Event, keyboard::Keycode, libc::printf, pixels::Color};
use specs::prelude::*;

use crate::components::{Collision, Drawable, DrawableType, InputMovement, Physics, PressedKeys};

pub fn create_rect(
    world: &mut World,
    position: [f32; 2],
    dimensions: [f32; 2],
    velocity: Option<[f32; 2]>,
    mass: Option<f32>,
    color: Option<Color>,
) -> EntityBuilder {
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
}

pub fn create_player(world: &mut World) -> Entity {
    create_rect(
        world,
        [400.0, 400.0],
        [50.0, 50.0],
        Some([0.0, 0.0]),
        Some(10.0),
        Some(Color::RGB(0, 255, 0)),
    )
    .with(InputMovement {
        handler: |ev: Event, p: &mut Physics, s: &mut InputMovement| {
            let relevant = vec![Keycode::W, Keycode::A, Keycode::S, Keycode::D];

            if ev.is_keyboard() == false {
                return false;
            }
            print!("Key event {:?}\n", ev);

            let keyup;

            let keycode = match ev {
                Event::KeyDown { keycode, .. } => {
                    keyup = false;
                    keycode.unwrap()
                }
                Event::KeyUp { keycode, .. } => {
                    keyup = true;
                    keycode.unwrap()
                }
                _ => return false,
            };

            if relevant.contains(&keycode) {
                if keyup {
                    s.pressed_relevant_keys.remove(&keycode);
                } else {
                    s.pressed_relevant_keys.insert(keycode);
                }
            }

            // calc new velocity
            let mut new_velocity = Vector2::new(0.0, 0.0);

            if s.pressed_relevant_keys.contains(&Keycode::W) {
                new_velocity.y -= s.directional_velocity;
            }

            if s.pressed_relevant_keys.contains(&Keycode::A) {
                new_velocity.x -= s.directional_velocity;
            }

            if s.pressed_relevant_keys.contains(&Keycode::S) {
                new_velocity.y += s.directional_velocity;
            }

            if s.pressed_relevant_keys.contains(&Keycode::D) {
                new_velocity.x += s.directional_velocity;
            }

            p.velocity = new_velocity;

            return true;
        },
        pressed_relevant_keys: PressedKeys::new(),
        directional_velocity: 500.0,
    })
    .build()
}
