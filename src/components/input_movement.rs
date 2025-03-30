use std::collections::HashSet;

use sdl3::event::Event;
use specs::Component;
use specs::prelude::*;

use super::Physics;

pub type PressedKeys = HashSet<sdl3::keyboard::Keycode>;

///
/// Component has input controlled movement
///
#[derive(Debug, Clone)]
pub struct InputMovement {
    /// Handler, return true if even was consumed
    pub handler: fn(ev: Event, p: &mut Physics, r: &mut InputMovement) -> bool,

    pub pressed_relevant_keys: PressedKeys,

    pub directional_velocity: f32,
}

impl Component for InputMovement {
    type Storage = HashMapStorage<Self>;
}
