use std::{mem, time::Instant};

use sdl3::{libc::rand, sys::stdinc::SDL_randf};
use specs::prelude::*;

use crate::{
    DebugSysState,
    components::{CollisionComp, CollisionResData, EntityType, PhysicsComp},
};

pub struct SysCollisionResolver;

impl<'a> System<'a> for SysCollisionResolver {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, CollisionResData>,
        ReadStorage<'a, CollisionComp>,
        WriteStorage<'a, PhysicsComp>,
        Write<'a, DebugSysState>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut collision_res_components,
            collision_components,
            mut physics_components,
            mut debug_sysState,
        ): Self::SystemData,
    ) {
        for entity in (&*entities).join() {
            let collision_resolution_info = collision_res_components.get(entity);

            if collision_resolution_info.is_none() {
                continue;
            }

            let collision_resolution_info = collision_resolution_info.unwrap().clone();

            collision_res_components.remove(entity);

            // make them immutable after this point
            // the entities area already sorted from the contact finding
            let entity_a = entity;
            let entity_b = collision_resolution_info.other;

            let entity_a_col = collision_components.get(entity_a).unwrap();
            let entity_b_col = collision_components.get(entity_b).unwrap();

            // physics stuff
            let physics_a = physics_components.get(entity_a).unwrap();
            let physics_b = physics_components.get(entity_a).unwrap();

            // the order of these will always be constant

            match entity_a_col.my_collision_type {
                EntityType::Enemy => match entity_b_col.my_collision_type {
                    EntityType::Enemy => {}
                    EntityType::EnemyBullet => {}
                    EntityType::Player => {}
                    EntityType::PlayerBullet => {
                        print!("damage enem\n")
                    }
                    EntityType::Wall => {
                        let physics = &mut physics_components.get_mut(entity_a).unwrap();

                        physics.physics.world_space_position -= collision_resolution_info
                            .vec_to_other
                            * collision_resolution_info.contact.dist.abs();
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

                        let mut offfset_direction = collision_resolution_info.contact.point1
                            - collision_resolution_info.contact.point2;

                        if offfset_direction.x == 0.0 && offfset_direction.y == 0.0 {
                            // this avoids a crash with a div by 0 error if the two points are exactly equal
                            offfset_direction = [1.0, 0.0].into();
                        }

                        let offfset_direction = offfset_direction.normalize();

                        physics.physics.world_space_position -=
                            offfset_direction * collision_resolution_info.contact.dist.abs();
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
