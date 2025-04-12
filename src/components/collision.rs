use std::time::Instant;

use bitmask_enum::bitmask;
use parry2d::query::Contact;
use specs::prelude::*;

#[bitmask()]
pub enum EntityType {
    Enemy,
    EnemyBullet,
    Player,
    PlayerBullet,
    Wall,
}

pub struct CollisionComp {
    pub collides_with: EntityType,
    pub my_collision_type: EntityType,
}

impl Component for CollisionComp {
    type Storage = VecStorage<Self>;
}

#[derive(Clone)]
pub struct CollisionResData {
    pub other: Entity,
    pub time_of_collision: Instant,
    pub contact: Contact
}

impl Component for CollisionResData {
    type Storage = VecStorage<Self>;
}
