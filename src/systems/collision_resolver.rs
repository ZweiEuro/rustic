use std::mem;

use specs::prelude::*;

use crate::components::{CollisionComp, CollisionResData, EntityType, PhysicsComp};

pub struct SysCollisionResolver;

impl<'a> System<'a> for SysCollisionResolver {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, CollisionResData>,
        ReadStorage<'a, CollisionComp>,
        WriteStorage<'a, PhysicsComp>,
    );

    fn run(
        &mut self,
        (entities, mut collision_res_components,  collision_components, mut physics_components): Self::SystemData,
    ) {
        for entity in (&*entities).join() {
            let collision_resolution_info = collision_res_components.get(entity);

            if collision_resolution_info.is_none() {
                continue;
            }

            let collision_resolution_info = collision_resolution_info.unwrap().clone();

            collision_res_components.remove(entity);

            let mut entity_a = entity;
            let mut entity_b = collision_resolution_info.other;

            {
                // safety check both are still live and sorted correctly

                if collision_components.get(entity_a).is_none()
                    || collision_components.get(entity_b).is_none()
                {
                    // objects got deleted or does not have a proper collision component anymore
                    continue;
                }

                let entity_a_col = collision_components.get(entity_a).unwrap();
                let entity_b_col = collision_components.get(entity_b).unwrap();

                // make a always be the smaller one
                if entity_a_col.my_collision_type > entity_b_col.my_collision_type {
                    mem::swap(&mut entity_a, &mut entity_b);
                }
            }

            // make them immutable after this point
            let entity_a = entity_a;
            let entity_b = entity_b;

            let entity_a_col = collision_components.get(entity_a).unwrap();
            let entity_b_col = collision_components.get(entity_b).unwrap();

            // the order of these will always be constant

            match entity_a_col.my_collision_type {
                EntityType::Enemy => match entity_b_col.my_collision_type {
                    EntityType::Enemy => {}
                    EntityType::EnemyBullet => {}
                    EntityType::Player => {}
                    EntityType::PlayerBullet => {
                        print!("damage enem\ny")
                    }
                    EntityType::Wall => {
                        let physics = &mut physics_components.get_mut(entity_a).unwrap();

                        let collision_direction = physics.physics.direction;

                        physics.physics.world_space_position -=
                            collision_direction * collision_resolution_info.contact.dist.abs();
                    }
                    _ => panic!("should never occur"),
                },
                EntityType::EnemyBullet => match entity_b_col.my_collision_type {
                    EntityType::EnemyBullet => {}
                    EntityType::Player => {
                        print!("damage player\n");
                        entities.delete(entity_a).unwrap()
                    }
                    EntityType::PlayerBullet => {}
                    EntityType::Wall => {
                        print!("Delete e bullet hit wall\n");
                        entities.delete(entity_a).unwrap()
                    }
                    _ => panic!("should never occur"),
                },
                EntityType::Player => match entity_b_col.my_collision_type {
                    EntityType::Player => {}
                    EntityType::PlayerBullet => {}
                    EntityType::Wall => {
                        print!("collide player with wall\n");

                        let physics = &mut physics_components.get_mut(entity_a).unwrap();

                        let collision_direction = physics.physics.direction;

                        physics.physics.world_space_position -=
                            collision_direction * collision_resolution_info.contact.dist.abs();
                    }
                    _ => panic!("should never occur"),
                },
                EntityType::PlayerBullet => match entity_b_col.my_collision_type {
                    EntityType::PlayerBullet => {}
                    EntityType::Wall => {
                        print!("Delete p bullet hit wall\n");
                        entities.delete(entity_a).unwrap()
                    }
                    _ => panic!("should never occur"),
                },
                EntityType::Wall => match entity_b_col.my_collision_type {
                    EntityType::Wall => {
                        panic!(" Wall hit wall ? ummm....")
                    }
                    _ => panic!("should never occur"),
                },
                _ => panic!("should never occur"),
            }
        }
    }
}
