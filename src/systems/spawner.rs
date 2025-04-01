use specs::prelude::*;

use crate::components::{Collision, Drawable, DrawableType, Physics, SpawnInformation, SpawnProperties};

#[derive(SystemData)]
pub struct SpawnerComp<'a> {
    spawn_properties: WriteStorage<'a, SpawnProperties>,
}

pub struct SysSpawner;

impl<'a> System<'a> for SysSpawner {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Physics>,
        WriteStorage<'a, Drawable>,
        WriteStorage<'a, Collision>,
        SpawnerComp<'a>,
    );

    fn run(
        &mut self,
        (entites, mut physics_comp, mut  drawable_comp, mut collision_comp, mut data): Self::SystemData,
    ) {
        for (entity, obj) in (&entites, &mut data.spawn_properties).join() {
            match obj.info {
                SpawnInformation::Bullet { physics, color } => {
                    // physics
                    physics_comp.insert(entity, physics).unwrap();
                
                    // drawing
                    physics_comp.insert(entity, Drawable {
                        shape: DrawableType::Rectangle { width: physics., height: () }
                    })

                
                }

                _ => todo!("property not set to spawn yet"),
            }

            print!("spawning obj {:?}\n", obj);

            entites.delete(entity).unwrap();
        }
    }
}
