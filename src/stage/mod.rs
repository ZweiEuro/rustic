mod camera;

use std::collections::HashSet;

pub use camera::Camera;
use glm::*;
use miniquad::{Bindings, KeyCode, MouseButton, Pipeline, RenderingBackend};

use crate::{shaders, textures};

pub struct WorldState {
    pub cam: Camera,

    pub model: glm::Mat4,
    pub view: glm::Mat4,
    pub projection: glm::Mat4,
}

pub struct Input {
    pub pressed_keys: HashSet<KeyCode>,
    pub pressed_mouse_buttons: HashSet<MouseButton>,

    pub prev_mouse_location: Vec2,
}
pub struct StageMetadata {
    pub last_time_update_fn_run: f64,
    pub time_stage_started: f64,
}

pub struct Settings {
    pub render_wireframe: bool,
    pub debug_toggle_1: bool,
    pub debug_toggle_2: bool,
    pub debug_toggle_3: bool,
    pub debug_toggle_4: bool,
}

pub struct Stage {
    pub ctx: Box<dyn RenderingBackend>,

    pub meta: StageMetadata,

    pub world: WorldState,

    pub pipeline: Pipeline,
    pub bindings: Bindings,

    pub settings: Settings,

    pub textures: Vec<textures::Texture>,
    pub shaders: Vec<shaders::Shader>,

    pub input: Input,
}
