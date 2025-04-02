use std::collections::HashSet;

use sdl3::event::Event;
use specs::Component;
use specs::prelude::*;

use super::PhysicsComp;

pub type PressedKeys = HashSet<sdl3::keyboard::Keycode>;

///
/// Component has input controlled movement
///
#[derive(Debug, Clone)]
pub struct KeyboardHandling {
    /// Handler, return true if even was consumed
    pub handler: fn(ev: Event, p: &mut PhysicsComp, r: &mut KeyboardHandling) -> bool,

    pub pressed_relevant_keys: PressedKeys,

    pub directional_velocity: f32,
}

impl Component for KeyboardHandling {
    type Storage = HashMapStorage<Self>;
}
