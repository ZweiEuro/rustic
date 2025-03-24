use std::time::Instant;

use specs::prelude::*;

use crate::components::Physics;

#[derive(SystemData)]
struct PhysicsComp<'a> {
    physics: WriteStorage<'a, Physics>,
}

struct SysMovement;

impl<'a> System<'a> for SysMovement {
    type SystemData = PhysicsComp<'a>;

    fn run(&mut self, mut data: PhysicsComp) {
        for physics in (&mut data.physics).join() {
            let time_delta = physics.last_time_updated.elapsed();

            physics.last_time_updated = Instant::now();

            physics.world_space_position += physics.velocity * time_delta.as_secs_f32();
        }
    }
}
