extern crate sdl3;

use components::{Collision, Drawable, DrawableType, Physics};

use parry2d::na::Vector2;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use specs::DispatcherBuilder;
use specs::{prelude::*, storage::HashMapStorage};
use std::f32::INFINITY;
use std::thread::Thread;
use std::time::{Duration, Instant};
use systems::{SysCollision, SysMovement, SysRender};
use world_objects::create_rect;

mod components;
mod systems;
mod world_objects;

use std::thread;

static MAIN_LOOP_FPS: i32 = 120;

pub fn create_game_objects(w: &mut World) {
    create_rect(
        w,
        [50.0, 50.0],
        [50.0, 50.0],
        Some([500.0, 0.0]),
        Some(2.0),
        None,
    );

    create_rect(w, [200.0, 50.0], [50.0, 50.0], Some([0.0, 0.0]), None, None);

    // world boundary
    create_rect(
        w,
        [10.0, 50.0],
        [10.0, 200.0],
        None,
        Some(INFINITY),
        Some(Color::RGB(0, 0, 0)),
    );

    create_rect(
        w,
        [500.0, 50.0],
        [10.0, 200.0],
        None,
        Some(INFINITY),
        Some(Color::RGB(0, 0, 0)),
    );
}
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

    create_game_objects(&mut w);

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
