use std::ops::Deref;

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
                    let v_a = objects[index_a].0.velocity;
                    let v_b = objects[index_comp].0.velocity;

                    let v_a_new = (v_a * (m_a - m_b) + 2. * m_b * v_b) / (m_a + m_b);
                    let v_b_new = (v_b * (m_b - m_a) + 2. * m_a * v_a) / (m_a + m_b);

                    objects[index_a].0.velocity = v_a_new;
                    objects[index_comp].0.velocity = v_b_new;

                    continue 'outer;
                }
            }
        }
    }
}
