use std::f32::INFINITY;

use parry2d::na::Vector2;
use sdl3::{event::Event, keyboard::Keycode, pixels::Color};
use specs::prelude::*;

use crate::components::{
    CollisionComp, DrawableComp, EntityType, KeyboardHandling, Physics, PhysicsComp, PressedKeys,
    Shape,
};

pub fn create_rect(
    world: &mut World,
    position: [f32; 2],
    dimensions: [f32; 2],
    direction: Option<[f32; 2]>,
    speed: Option<f32>,
    mass: Option<f32>,
    color: Option<Color>,
) -> EntityBuilder {
    let physics = Physics {
        world_space_position: Vector2::new(position[0], position[1]),
        direction: direction.map_or(Vector2::new(1.0, 0.0), |v| Vector2::new(v[0], v[1])),
        speed: speed.unwrap_or(0.0),
        mass: mass.unwrap_or(1.0),
        shape: Shape::Rectangle {
            width: dimensions[0],
            height: dimensions[1],
        },
    };

    let mut drawable: DrawableComp = physics.into();
    drawable.color = color.unwrap_or(drawable.color);

    let coll: CollisionComp = CollisionComp {
        collides_with: EntityType::all_bits(),
        my_collision_type: EntityType::Enemy,
    };

    world
        .create_entity()
        .with(PhysicsComp::new(physics))
        .with(drawable)
        .with(coll)
}

pub fn create_player(world: &mut World) -> Entity {
    let handler = |ev: Event, p: &mut PhysicsComp, s: &mut KeyboardHandling| {
        let relevant = vec![Keycode::W, Keycode::A, Keycode::S, Keycode::D];

        if ev.is_keyboard() {
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
            let mut new_direction_vector = Vector2::new(0.0, 0.0);

            if s.pressed_relevant_keys.contains(&Keycode::W) {
                new_direction_vector += Vector2::new(0.0, -1.0);
            }

            if s.pressed_relevant_keys.contains(&Keycode::A) {
                new_direction_vector += Vector2::new(-1.0, 0.0);
            }

            if s.pressed_relevant_keys.contains(&Keycode::S) {
                new_direction_vector += Vector2::new(0.0, 1.0);
            }

            if s.pressed_relevant_keys.contains(&Keycode::D) {
                new_direction_vector += Vector2::new(1.0, 0.0);
            }

            p.physics.direction = new_direction_vector;

            if p.physics.direction.magnitude() != 0.0 {
                p.physics.speed = s.directional_velocity;
            }
            return true;
        }

        return false;
    };

    create_rect(
        world,
        [400.0, 400.0],
        [50.0, 50.0],
        None,
        None,
        Some(10.0),
        Some(Color::RGB(0, 255, 0)),
    )
    .with(KeyboardHandling {
        handler,
        pressed_relevant_keys: PressedKeys::new(),
        directional_velocity: 500.0,
    })
    .build()
}
