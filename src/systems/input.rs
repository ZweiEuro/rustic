use parry2d::na::Vector2;
use sdl3::{EventPump, event::Event, keyboard::Keycode, mouse::MouseButton, pixels::Color};
use specs::prelude::*;

use crate::{
    PlayerEntity, SysState,
    components::{
        EntityType, KeyboardHandling, Physics, PhysicsComp, SpawnInformation, SpawnProperties_comp,
    },
};

#[derive(SystemData)]
pub struct Data<'a> {
    sys_state: Write<'a, SysState>,
    move_handler: WriteStorage<'a, KeyboardHandling>,
}

pub struct SysInput {
    pub event_pump: EventPump,
}

unsafe impl Sync for SysInput {}
unsafe impl Send for SysInput {}

impl<'a> System<'a> for SysInput {
    type SystemData = (
        Read<'a, PlayerEntity>,                 // resource
        Entities<'a>,                           // base
        WriteStorage<'a, SpawnProperties_comp>, // component
        WriteStorage<'a, PhysicsComp>,          // component
        Data<'a>,                               // actual data
    );

    fn run(
        &mut self,
        (
            player_entity,
            entities,
            mut spawn_properties_components,
            mut physics_components,
            mut data,
        ): Self::SystemData,
    ) {
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

            for obj in (&entities, &mut data.move_handler).join() {
                if let Some(ph) = physics_components.get_mut(obj.0) {
                    if (obj.1.handler)(event.clone(), ph, obj.1) == true {
                        break;
                    }
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

                        if player_entity.entity.is_none() {
                            print!("no player to spawn anything on click");
                            continue;
                        }

                        let click_pos = Vector2::new(x, y);

                        let player_entity = player_entity.entity.unwrap();
                        let player_physics = physics_components.get(player_entity).unwrap();

                        let bullet_direction =
                            (click_pos - player_physics.physics.world_space_position).normalize();

                        let bullet_spawn_pos =
                            player_physics.physics.world_space_position + bullet_direction * 100.0;

                        let bullet = entities.create();

                        spawn_properties_components
                            .insert(
                                bullet,
                                SpawnProperties_comp::new({
                                    SpawnInformation {
                                        entity_type: EntityType::PlayerBullet,
                                        physics: Some(Physics {
                                            world_space_position: bullet_spawn_pos,
                                            direction: bullet_direction,
                                            speed: 600.0,
                                            mass: 0.001,
                                            shape: crate::components::Shape::Rectangle {
                                                width: 5.0,
                                                height: 5.0,
                                            },
                                        }),
                                        color: Some(Color::RGB(0, 0, 255)),
                                    }
                                }),
                            )
                            .unwrap();
                    }
                    _ => {}
                }
            }
        }
    }
}
