use std::{f32::INFINITY, ops::Deref};

use parry2d::{na::Isometry2, query::intersection_test};
use specs::prelude::*;

use crate::components::{CollisionComp, PhysicsComp};

#[derive(SystemData)]
pub struct Data<'a> {
    physics: WriteStorage<'a, PhysicsComp>,
    collision: ReadStorage<'a, CollisionComp>,
}

pub struct SysCollision;

impl<'a> System<'a> for SysCollision {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Data) {
        // check every entity against every other entity

        let mut objects = (&mut data.physics, &data.collision)
            .join()
            .collect::<Vec<_>>();

        'outer: for index_a in 0..objects.len() {
            for index_comp in (index_a + 1)..objects.len() {
                if index_a == index_comp {
                    continue;
                }
                let a_pos = &objects[index_a].0.physics.world_space_position;
                let b_pos = &objects[index_comp].0.physics.world_space_position;

                let res = intersection_test(
                    &Isometry2::new(*a_pos, 0.),
                    objects[index_a].1.collision_shape.deref(),
                    &Isometry2::new(*b_pos, 0.),
                    objects[index_comp].1.collision_shape.deref(),
                )
                .unwrap();

                if res {
                    let (left, right) = objects.split_at_mut(index_comp);

                    let left_len = left.len();

                    // get two mutable instances of the physics we are editing
                    let physics_a = &mut (left.get_mut(index_a).unwrap().0.physics);
                    let physics_b = &mut (right.get_mut(index_comp - left_len).unwrap().0.physics);

                    let m_a = physics_a.mass;
                    let m_b = physics_b.mass;

                    // if one of the objects has infinite mass, it is considered immovable
                    // turn the other object around
                    if m_a == INFINITY {
                        physics_b.direction = -physics_b.direction;
                        continue 'outer;
                    } else if m_b == INFINITY {
                        physics_a.direction = -physics_a.direction;
                        continue 'outer;
                    }

                    let v_a = physics_b.direction * physics_b.speed;
                    let v_b = physics_a.direction * physics_a.speed;

                    let v_a_new = (v_a * (m_a - m_b) + 2. * m_b * v_b) / (m_a + m_b);
                    let v_b_new = (v_b * (m_b - m_a) + 2. * m_a * v_a) / (m_a + m_b);

                    // if the speed is 0, then the normalization will fail
                    // the direction will just stay the way it was before the collision

                    physics_b.speed = v_a_new.magnitude();

                    if physics_b.speed != 0. {
                        physics_b.direction = v_a_new.normalize();
                    }

                    assert!(physics_b.direction.x.is_finite() && physics_b.direction.y.is_finite());

                    physics_a.speed = v_b_new.magnitude();

                    if physics_a.speed != 0. {
                        physics_a.direction = v_b_new.normalize();
                    }

                    assert!(physics_a.direction.x.is_finite() && physics_a.direction.y.is_finite());

                    continue 'outer;
                }
            }
        }
    }
}
