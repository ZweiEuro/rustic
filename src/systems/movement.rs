use std::time::{Duration, Instant};

use crate::components::PhysicsComp;
use specs::prelude::*;

#[derive(SystemData)]
pub struct Data<'a> {
    physics: WriteStorage<'a, PhysicsComp>,
}
pub struct SysMovement;

impl<'a> System<'a> for SysMovement {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Data) {
        for physics_component in (&mut data.physics).join() {
            let time_delta = physics_component.last_time_updated.elapsed();
            physics_component.last_time_updated = Instant::now();

            if time_delta > Duration::from_secs(1) {
                print!("Movement system had more than 1s delay!");
                return;
            }

            physics_component.physics.world_space_position += physics_component.physics.direction
                * physics_component.physics.speed
                * time_delta.as_secs_f32();

            assert!(
                physics_component.physics.world_space_position.x.is_finite()
                    && physics_component.physics.world_space_position.y.is_finite()
            );
        }
    }
}
