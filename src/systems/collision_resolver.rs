use specs::prelude::*;

use crate::components::{CollisionComp, CollisionResData, EntityType};

pub struct SysCollisionResolver;

impl<'a> System<'a> for SysCollisionResolver {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, CollisionResData>,
        ReadStorage<'a, CollisionComp>,
    );

    fn run(
        &mut self,
        (entities, mut collision_res_component, mut collision_component): Self::SystemData,
    ) {
        for entity in (&*entities).join() {
            let t = collision_res_component.get_mut(entity);

            if t.is_none() {
                continue;
            }

            let t = t.unwrap();

            let entity_a = entity;
            let entity_b = t.other;

            let mut collision_components = [
                collision_component.get(entity_a).unwrap(),
                collision_component.get(entity_b).unwrap(),
            ];

            collision_components.sort_by(|a, b| a.my_collision_type.cmp(&b.my_collision_type));

            let entity_a_col = collision_components[0];
            let entity_b_col = collision_components[1];

            // the order of these will always be constant
            match entity_b_col.my_collision_type {
                EntityType::Enemy => {}
                EntityType::EnemyBullet => {}
                EntityType::Player => {}
                EntityType::PlayerBullet => {}
                EntityType::Wall => {}
                _ => panic!("unknown entity type "),
            }

            match entity_a_col.my_collision_type {
                EntityType::Enemy => match entity_b_col.my_collision_type {
                    EntityType::Enemy => {}
                    EntityType::EnemyBullet => {}
                    EntityType::Player => {}
                    EntityType::PlayerBullet => {
                        print!("damage enem\ny")
                    }
                    EntityType::Wall => {
                        print!("collide enemy with wall\n");
                    }
                    _ => panic!("should never occur"),
                },
                EntityType::EnemyBullet => match entity_b_col.my_collision_type {
                    EntityType::EnemyBullet => {}
                    EntityType::Player => {
                        print!("damage player\n");
                        entities.delete(entity_a).unwrap();
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

            collision_res_component.remove(entity);
        }
    }
}
