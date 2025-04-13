extern crate sdl3;

use components::CollisionComp;
use components::EntityType;
use parry2d::query::Contact;
use sdl3::pixels::Color;
use specs::DispatcherBuilder;
use specs::prelude::*;
use std::f32::INFINITY;
use std::time::Duration;
use std::time::Instant;
use systems::SysCollisionResolver;
use systems::SysSpawner;
use systems::{SysCollision, SysInput, SysMovement, SysRender};
use world_objects::{create_player, create_rect};

mod components;
mod systems;
mod world_objects;

use std::thread;

static MAIN_LOOP_FPS: i32 = 120;

pub fn create_game_objects(world: &mut World) {
    let coll: CollisionComp = CollisionComp {
        collides_with: EntityType::all_bits(),
        my_collision_type: EntityType::Enemy,
    };

    create_rect(
        world,
        [50.0, 50.0],
        [50.0, 50.0],
        None,
        Some(500.0),
        Some(2.0),
        None,
    )
    .with(coll)
    .build();

    let coll: CollisionComp = CollisionComp {
        collides_with: EntityType::all_bits(),
        my_collision_type: EntityType::Enemy,
    };

    create_rect(
        world,
        [200.0, 50.0],
        [50.0, 50.0],
        Some([1.0, 0.0]),
        None,
        None,
        None,
    )
    .with(coll)
    .build();

    // world boundary
    let coll: CollisionComp = CollisionComp {
        collides_with: EntityType::all_bits(),
        my_collision_type: EntityType::Wall,
    };

    create_rect(
        world,
        [10.0, 50.0],
        [10.0, 200.0],
        None,
        None,
        Some(INFINITY),
        Some(Color::RGB(0, 0, 0)),
    )
    .with(coll)
    .build();

    let coll: CollisionComp = CollisionComp {
        collides_with: EntityType::all_bits(),
        my_collision_type: EntityType::Wall,
    };

    create_rect(
        world,
        [500.0, 50.0],
        [10.0, 200.0],
        None,
        None,
        Some(INFINITY),
        Some(Color::RGB(0, 0, 0)),
    )
    .with(coll)
    .build();

    let player = create_player(world);

    world.insert(PlayerEntity {
        entity: Some(player),
    });
}

#[derive(Default)]
pub struct SysState {
    running: bool,
}

#[derive(Default)]
pub struct DebugSysState {}

#[derive(Default)]

pub struct PlayerEntity {
    entity: Option<Entity>,
}

pub fn main() {
    let mut world = World::new();

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

    let dispatcher_builder = DispatcherBuilder::new()
        .with(SysMovement, "movement", &[])
        .with(SysCollision, "collision", &["movement"])
        .with(
            SysInput {
                event_pump: sdl_context.event_pump().unwrap(),
            },
            "input",
            &[],
        )
        .with(SysSpawner, "spawner", &["input"])
        .with(SysCollisionResolver, "collision_resolver", &["collision"])
        .with_thread_local(SysRender {
            canvas: window.into_canvas(),
        });

    world.insert(SysState { running: true });
    world.insert(DebugSysState::default());

    let mut dispatcher = dispatcher_builder.build();
    dispatcher.setup(&mut world);

    create_game_objects(&mut world);

    'running: loop {
        dispatcher.dispatch(&world);
        world.maintain();

        if world.read_resource::<SysState>().running == false {
            break 'running;
        }

        thread::sleep(Duration::from_millis(
            ((1.0 / MAIN_LOOP_FPS as f32) * 1000.0) as u64,
        ));
    }
}
