extern crate sdl3;

use components::{Collision, Drawable, DrawableType, Physics};

use parry2d::na::Vector2;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use specs::DispatcherBuilder;
use specs::{prelude::*, storage::HashMapStorage};
use std::thread::Thread;
use std::time::{Duration, Instant};
use systems::{SysCollision, SysMovement, SysRender};

mod components;
mod systems;

use std::thread;

static MAIN_LOOP_FPS: i32 = 120;

pub fn main() {
    print!("Hello, world!\n");
    let mut w = World::new();

    // SDL stuff

    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("ECS test", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // Register systems

    let mut dispatcher_builder = DispatcherBuilder::new()
        .with(SysMovement, "movement", &[])
        .with(SysCollision, "collision", &[])
        .with_thread_local(SysRender {
            canvas: window.into_canvas(),
        });

    let mut dispatcher = dispatcher_builder.build();
    dispatcher.setup(&mut w);

    w.create_entity()
        .with(Physics {
            world_space_position: Vector2::new(50.0, 50.0),
            velocity: Vector2::new(100.0, 0.0),
            mass: 1.0,
            last_time_updated: Instant::now(),
        })
        .with(Drawable {
            drawable_type: DrawableType::Rectangle,
            color: Color::RGB(255, 0, 0),
            width: 50.0,
            height: 50.0,
            radius: 0.0,
        })
        .with(Collision {
            collision_shape: Box::new(parry2d::shape::Cuboid::new(Vector2::new(25.0, 25.0))),
        })
        .build();

    w.create_entity()
        .with(Physics {
            world_space_position: Vector2::new(200.0, 50.0),
            velocity: Vector2::new(0.0, 0.0),
            mass: 1.0,
            last_time_updated: Instant::now(),
        })
        .with(Drawable {
            drawable_type: DrawableType::Rectangle,
            color: Color::RGB(255, 0, 0),
            width: 50.0,
            height: 50.0,
            radius: 0.0,
        })
        .with(Collision {
            collision_shape: Box::new(parry2d::shape::Cuboid::new(Vector2::new(25.0, 25.0))),
        })
        .build();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        dispatcher.dispatch(&w);
        w.maintain();

        thread::sleep(Duration::from_millis(
            ((1.0 / MAIN_LOOP_FPS as f32) * 1000.0) as u64,
        ));
    }
}
