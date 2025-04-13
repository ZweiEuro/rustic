use std::{ops::Deref, time::Instant};

use parry2d::{
    na::{Isometry2, Vector2},
    query::{self, Contact, intersection_test},
};
use sdl3::sys::assert;
use specs::prelude::*;

use crate::components::{CollisionComp, CollisionResData, PhysicsComp, Shape};

#[derive(SystemData)]
pub struct Data<'a> {
    physics: WriteStorage<'a, PhysicsComp>,
    collision: ReadStorage<'a, CollisionComp>,
}

pub struct SysCollision;

// broad phase of collision detection
impl<'a> System<'a> for SysCollision {
    type SystemData = (Entities<'a>, WriteStorage<'a, CollisionResData>, Data<'a>);

    fn run(&mut self, (entities, mut collisionDataComp, mut data): Self::SystemData) {
        // check every entity against every other entity

        let mut objects = (&*entities, &mut data.physics, &data.collision)
            .join()
            .collect::<Vec<_>>();

        'outer: for index_a in 0..objects.len() {
            for index_b in (index_a + 1)..objects.len() {
                if index_a == index_b {
                    continue;
                }

                // we have to do this with mutable splits else it wont know that they are editable at once
                let (left, right) = objects.split_at_mut(index_b);

                let left_len = left.len();

                // get two mutable instances of the physics we are editing
                let object_a = &mut (left.get_mut(index_a).unwrap());
                let object_b = &mut (right.get_mut(index_b - left_len).unwrap());

                // check the collision mask to check if they collide
                if object_a
                    .2
                    .collides_with
                    .intersects(object_b.2.my_collision_type)
                    == false
                {
                    // we should not check for intersect
                    continue;
                }

                // build both collision objects and then collide them

                let coll_a: Box<dyn parry2d::shape::Shape> = match object_a.1.physics.shape {
                    Shape::Rectangle { width, height } => Box::new(parry2d::shape::Cuboid::new(
                        Vector2::new(width / 2.0, height / 2.0),
                    )),
                    Shape::Circle { radius } => Box::new(parry2d::shape::Ball::new(radius)),

                    _ => panic!("unknown shape!"),
                };

                let coll_b: Box<dyn parry2d::shape::Shape> = match object_b.1.physics.shape {
                    Shape::Rectangle { width, height } => Box::new(parry2d::shape::Cuboid::new(
                        Vector2::new(width / 2.0, height / 2.0),
                    )),
                    Shape::Circle { radius } => Box::new(parry2d::shape::Ball::new(radius)),

                    _ => panic!("unknown shape!"),
                };

                let prediction = 1.0;

                // do the actual collision check

                // sort the collision object so its always a -> b in proper entity order

                let res: Option<Contact>;

                // collision should always be from a -> b , from smaller to larger type number
                if object_a.2.my_collision_type > object_b.2.my_collision_type {
                    res = query::contact(
                        &Isometry2::new(object_b.1.physics.world_space_position, 0.),
                        coll_b.deref(),
                        &Isometry2::new(object_a.1.physics.world_space_position, 0.),
                        coll_a.deref(),
                        prediction,
                    )
                    .unwrap();

                    if let Some(contact) = res {
                        // there should never be two collisions in the same pass
                        assert!(collisionDataComp.get(object_b.0).is_none());

                        collisionDataComp
                            .insert(
                                object_b.0,
                                CollisionResData {
                                    other: object_a.0,
                                    time_of_collision: Instant::now(),
                                    contact,
                                    vec_to_other: (object_a.1.physics.world_space_position
                                        - object_b.1.physics.world_space_position)
                                        .normalize(),
                                },
                            )
                            .unwrap();
                    }
                } else {
                    res = query::contact(
                        &Isometry2::new(object_a.1.physics.world_space_position, 0.),
                        coll_a.deref(),
                        &Isometry2::new(object_b.1.physics.world_space_position, 0.),
                        coll_b.deref(),
                        prediction,
                    )
                    .unwrap();

                    if let Some(contact) = res {
                        assert!(collisionDataComp.get(object_a.0).is_none());

                        collisionDataComp
                            .insert(
                                object_a.0,
                                CollisionResData {
                                    other: object_b.0,
                                    time_of_collision: Instant::now(),
                                    contact,
                                    vec_to_other: (object_b.1.physics.world_space_position
                                        - object_a.1.physics.world_space_position)
                                        .normalize(),
                                },
                            )
                            .unwrap();
                    }
                }
            }
        }
    }
}
