use specs::prelude::*;

use crate::components::CollisionResData;

pub struct SysCollisionResolver;

impl<'a> System<'a> for SysCollisionResolver {
    type SystemData = (Entities<'a>, WriteStorage<'a, CollisionResData>);

    fn run(&mut self, (entities, mut collision_res_component): Self::SystemData) {
        for entity in (&*entities).join() {
            let t = collision_res_component.get_mut(entity);

            if t.is_none() {
                continue;
            }

            let t = t.unwrap();

            print!(
                "collision, component {:?} collided with {:?}\n",
                entity, t.other
            );
            collision_res_component.remove(entity);
        }
    }
}
