use specs::{prelude::*, rayon::spawn};

use crate::components::{
    CollisionComp, DrawableComp, EntityType, PhysicsComp, SpawnInformation, SpawnProperties_comp,
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

            match spawner_props.info.entity_type {
                EntityType::PlayerBullet => {
                    // Will crash if any of the info is missing, which is a good thing
                    let physics = spawner_props.info.physics.unwrap();
                    let color = spawner_props.info.color.unwrap();

                    // physics
                    physics_comps
                        .insert(entity, PhysicsComp::new(physics))
                        .unwrap();

                    // drawing
                    let mut drawable: DrawableComp = physics.into();
                    drawable.color = color;
                    drawable_comps.insert(entity, drawable).unwrap();

                    // collision
                    collision_comp
                        .insert(
                            entity,
                            CollisionComp {
                                collides_with: EntityType::Enemy,
                                my_collision_type: EntityType::PlayerBullet,
                            },
                        )
                        .unwrap();
                }

                _ => todo!("property not set to spawn yet"),
            }

            spawnerprops_comps.remove(entity);
        }
    }
}
