use std::time::Instant;

use parry2d::{math::Vector, na::Vector2};
use sdl3::{
    EventPump, event::Event, keyboard::Keycode, libc::printf, mouse::MouseButton, pixels::Color,
};
use specs::prelude::*;

use crate::{
    SysState,
    components::{
        Collision, Drawable, KeyboardHandling, Physics, SpawnInformation, SpawnProperties,
    },
};

#[derive(SystemData)]
pub struct InputComp<'a> {
    sys_state: Write<'a, SysState>,
    physics: WriteStorage<'a, Physics>,
    move_handler: WriteStorage<'a, KeyboardHandling>,
}

pub struct SysInput {
    pub event_pump: EventPump,
}

unsafe impl Sync for SysInput {}
unsafe impl Send for SysInput {}

impl<'a> System<'a> for SysInput {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, SpawnProperties>,
        InputComp<'a>,
    );

    fn run(&mut self, (entities, mut spawn_properties, mut data): Self::SystemData) {
        let mut sys_state = data.sys_state;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    sys_state.running = false;
                }
                _ => {}
            }

            for obj in (&mut data.physics, &mut data.move_handler).join() {
                if (obj.1.handler)(event.clone(), obj.0, obj.1) == true {
                    break;
                }
            }

            if event.is_mouse() {
                match event {
                    Event::MouseMotion { .. } => {}
                    Event::MouseButtonDown {
                        x, y, mouse_btn, ..
                    } => {
                        if mouse_btn != MouseButton::Left {
                            continue;
                        }

                        print!("Mouse button down at: x: {}, y: {}\n", x, y);

                        let bullet = entities.create();

                        spawn_properties
                            .insert(
                                bullet,
                                SpawnProperties::new({
                                    SpawnInformation::Bullet {
                                        physics: Physics {
                                            world_space_position: Vector2::new(x as f32, y as f32),
                                            direction: Vector2::new(1.0, 0.0),
                                            speed: 100.0,
                                            mass: 1.0,
                                            last_time_updated: Instant::now(),
                                        },
                                        color: Color::RGB(0, 0, 255),
                                    }
                                }),
                            )
                            .unwrap();

                        print!("created entity {:?}\n", bullet)
                    }
                    _ => {}
                }
            }
        }
    }
}
