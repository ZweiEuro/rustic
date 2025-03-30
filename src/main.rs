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
use systems::{SysCollision, SysInput, SysMovement, SysRender};
use world_objects::{create_player, create_rect};

mod components;
mod systems;
mod world_objects;

use std::thread;

static MAIN_LOOP_FPS: i32 = 120;

pub fn create_game_objects(world: &mut World) {
    create_rect(
        world,
        [50.0, 50.0],
        [50.0, 50.0],
        Some([500.0, 0.0]),
        Some(2.0),
        None,
    )
    .build();

    create_rect(
        world,
        [200.0, 50.0],
        [50.0, 50.0],
        Some([0.0, 0.0]),
        None,
        None,
    )
    .build();

    // world boundary
    create_rect(
        world,
        [10.0, 50.0],
        [10.0, 200.0],
        None,
        Some(INFINITY),
        Some(Color::RGB(0, 0, 0)),
    )
    .build();

    create_rect(
        world,
        [500.0, 50.0],
        [10.0, 200.0],
        None,
        Some(INFINITY),
        Some(Color::RGB(0, 0, 0)),
    )
    .build();

    create_player(world);
}

#[derive(Default)]
pub struct SysState {
    running: bool,
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

    // Register systems

    let mut dispatcher_builder = DispatcherBuilder::new()
        .with(SysMovement, "movement", &[])
        .with(SysCollision, "collision", &[])
        .with(
            SysInput {
                event_pump: sdl_context.event_pump().unwrap(),
            },
            "input",
            &[],
        )
        .with_thread_local(SysRender {
            canvas: window.into_canvas(),
        });

    w.insert(SysState { running: true });

    let mut dispatcher = dispatcher_builder.build();
    dispatcher.setup(&mut w);

    create_game_objects(&mut w);

    'running: loop {
        dispatcher.dispatch(&w);
        w.maintain();

        if (w.read_resource::<SysState>().running == false) {
            break 'running;
        }

        thread::sleep(Duration::from_millis(
            ((1.0 / MAIN_LOOP_FPS as f32) * 1000.0) as u64,
        ));
    }
}
