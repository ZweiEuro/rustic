use sdl3::pixels::Color;
use specs::prelude::*;

use super::{EntityType, Physics};

#[derive(Debug)]

pub struct SpawnInformation {
    pub entity_type: EntityType,
    pub physics: Option<Physics>,
    pub color: Option<Color>,
}

#[derive(Debug)]
pub struct SpawnProperties_comp {
    pub info: SpawnInformation,
}

impl SpawnProperties_comp {
    pub fn new(info: SpawnInformation) -> Self {
        Self { info }
    }
}

impl Component for SpawnProperties_comp {
    type Storage = HashMapStorage<Self>;
}
