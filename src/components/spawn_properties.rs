use sdl3::pixels::Color;
use specs::prelude::*;

use super::Physics;

#[derive(Debug)]

pub enum SpawnInformation {
    Bullet { physics: Physics, color: Color },
}

#[derive(Debug)]
pub struct SpawnProperties {
    pub info: SpawnInformation,
}

impl SpawnProperties {
    pub fn new(info: SpawnInformation) -> Self {
        Self { info }
    }
}

impl Component for SpawnProperties {
    type Storage = HashMapStorage<Self>;
}
