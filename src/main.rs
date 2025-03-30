extern crate sdl3;

use components::{Collision, Drawable, DrawableType, Physics};
use object::cube::Cube;
use object::object::PhysicsUpdated;
use object::{add_object, draw_all};
use parry2d::na::Vector2;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use specs::DispatcherBuilder;
use specs::{prelude::*, storage::HashMapStorage};
use std::thread::Thread;
use std::time::{Duration, Instant};
use systems::{SysMovement, SysRender};

mod components;
mod object;
mod systems;

use std::thread;

static MAIN_LOOP_FPS: i32 = 1;

pub fn main() {
    print!("Hello, world!\n");
    let mut w = World::new();

    // SDL stuff

    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("ECS test", 800, 600)
        .position_centered()
        .vulkan()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    print!(
        "Created context in thread id: {:?}\n",
        std::thread::current().id()
    );

    // Register systems

    let mut dispatcher_builder = DispatcherBuilder::new()
        .with(SysMovement, "movement", &[])
        .with(
            SysRender {
                canvas: window.into_canvas(),
            },
            "renderer",
            &[],
        );

    let mut dispatcher = dispatcher_builder.build();
    dispatcher.setup(&mut w);

    w.create_entity()
        .with(Physics {
            world_space_position: Vector2::new(0.0, 0.0),
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

    return;

    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("ECS test", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let cube = Cube::new(
        Vector2::new(100.0, 100.0),
        Vector2::new(50.0, 50.0),
        Vector2::new(200.0, -100.0),
        false,
    );

    let wall = Cube::new(
        Vector2::new(0.0, 0.0),
        Vector2::new(800.0, 10.0),
        Vector2::new(0.0, 0.0),
        true,
    );

    add_object(cube);
    add_object(wall);

    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        draw_all(&mut canvas);

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

        canvas.present();

        thread::sleep(Duration::from_millis(
            ((1.0 / MAIN_LOOP_FPS as f32) * 1000.0) as u64,
        ));
    }
}
