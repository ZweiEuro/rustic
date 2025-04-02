use specs::{prelude::*, rayon::spawn};

use crate::components::{
    CollisionComp, DrawableComp, PhysicsComp, SpawnInformation, SpawnProperties_comp,
};

pub struct SysSpawner;

impl<'a> System<'a> for SysSpawner {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, PhysicsComp>,
        WriteStorage<'a, DrawableComp>,
        WriteStorage<'a, CollisionComp>,
        WriteStorage<'a, SpawnProperties_comp>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut physics_comps,
            mut drawable_comps,
            mut collision_comp,
            mut spawnerprops_comps,
        ): Self::SystemData,
    ) {
        for entity in entities.join() {
            let spawner_props = spawnerprops_comps.get(entity);

            if spawner_props.is_none() {
                continue;
            }

            let spawner_props = spawner_props.unwrap();

            match spawner_props.info {
                SpawnInformation::Bullet { physics, color } => {
                    // physics
                    physics_comps
                        .insert(entity, PhysicsComp::new(physics))
                        .unwrap();

                    // drawing
                    let mut drawable: DrawableComp = physics.into();
                    drawable.color = color;
                    drawable_comps.insert(entity, drawable).unwrap();

                    // collision
                    let coll = physics.into();
                    collision_comp.insert(entity, coll).unwrap();
                }

                _ => todo!("property not set to spawn yet"),
            }

            print!("spawning obj {:?}\n", spawner_props);
        }
    }
}
