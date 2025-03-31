use std::{f32::INFINITY, ops::Deref};

use parry2d::{
    na::{Isometry2, Vector2},
    query::intersection_test,
};
use specs::prelude::*;

use crate::components::{Collision, Physics};

#[derive(SystemData)]
pub struct PhysicsComp<'a> {
    physics: WriteStorage<'a, Physics>,
    collision: ReadStorage<'a, Collision>,
}

pub struct SysCollision;

impl<'a> System<'a> for SysCollision {
    type SystemData = PhysicsComp<'a>;

    fn run(&mut self, mut data: PhysicsComp) {
        // check every entity against every other entity

        let mut objects = (&mut data.physics, &data.collision)
            .join()
            .collect::<Vec<_>>();

        'outer: for index_a in 0..objects.len() {
            for index_comp in (index_a + 1)..objects.len() {
                if index_a == index_comp {
                    continue;
                }
                let a_pos = &objects[index_a].0.world_space_position;
                let b_pos = &objects[index_comp].0.world_space_position;

                let res = intersection_test(
                    &Isometry2::new(*a_pos, 0.),
                    objects[index_a].1.collision_shape.deref(),
                    &Isometry2::new(*b_pos, 0.),
                    objects[index_comp].1.collision_shape.deref(),
                )
                .unwrap();

                if res {
                    let m_a = objects[index_a].0.mass;
                    let m_b = objects[index_comp].0.mass;

                    // if one of the objects has infinite mass, it is considered immovable
                    // turn the other object around
                    if m_a == INFINITY {
                        objects[index_comp].0.direction = -objects[index_comp].0.direction;
                        continue 'outer;
                    } else if m_b == INFINITY {
                        objects[index_a].0.direction = -objects[index_a].0.direction;
                        continue 'outer;
                    }

                    let v_a = objects[index_a].0.direction * objects[index_a].0.speed;
                    let v_b = objects[index_comp].0.direction * objects[index_comp].0.speed;

                    let v_a_new = (v_a * (m_a - m_b) + 2. * m_b * v_b) / (m_a + m_b);
                    let v_b_new = (v_b * (m_b - m_a) + 2. * m_a * v_a) / (m_a + m_b);

                    // if the speed is 0, then the normalization will fail
                    // the direction will just stay the way it was before the collision

                    objects[index_a].0.speed = v_a_new.magnitude();

                    if objects[index_a].0.speed != 0. {
                        objects[index_a].0.direction = v_a_new.normalize();
                    }

                    assert!(
                        objects[index_a].0.direction.x.is_finite()
                            && objects[index_a].0.direction.y.is_finite()
                    );

                    objects[index_comp].0.speed = v_b_new.magnitude();

                    if objects[index_comp].0.speed != 0. {
                        objects[index_comp].0.direction = v_b_new.normalize();
                    }

                    assert!(
                        objects[index_comp].0.direction.x.is_finite()
                            && objects[index_comp].0.direction.y.is_finite()
                    );

                    continue 'outer;
                }
            }
        }
    }
}
