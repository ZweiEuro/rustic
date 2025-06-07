mod camera;
pub use camera::Camera;
use miniquad::{
    Bindings, KeyCode, KeyMods, MouseButton, Pipeline, RenderingBackend, date, window,
};

use crate::{shaders, textures};

pub struct WorldState {
    pub cam: Camera,
}

pub struct StageMetadata {
    pub last_time_update_fn_run: f64,
    pub _time_stage_started: f64,
}

pub struct Settings {
    // pitch and yaw change per pixel moved
    pub mouse_sensitivity: f32,

    // debug options
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
    pub shaders: Vec<shaders::ShaderFile>,

    pub input: input::InputData,
}

// handle updates of various components
impl Stage {
    pub fn update(&mut self) {
        for shader in self.shaders.iter_mut() {
            if shader.reload_if_needed() {
                // println!("reload!");
            }
        }

        // a lot of update loops require some kind of time delta
        let delta = date::now() - self.meta.last_time_update_fn_run;

        self.update_camera(delta as f32);

        self.meta.last_time_update_fn_run = date::now();
    }

    pub fn update_camera(&mut self, update_delta: f32) {
        let pressed_keys = self.input.pressed_keys.clone();

        // forward and back
        if pressed_keys.contains(&KeyCode::W) {
            self.world.cam.move_forward(update_delta as f32);
        }

        if pressed_keys.contains(&KeyCode::S) {
            self.world.cam.move_backwards(update_delta as f32);
        }

        // left and right
        if pressed_keys.contains(&KeyCode::A) {
            self.world.cam.move_left(update_delta as f32);
        }

        if pressed_keys.contains(&KeyCode::D) {
            self.world.cam.move_right(update_delta as f32);
        }

        // up and down
        if pressed_keys.contains(&KeyCode::Space) {
            self.world.cam.move_up(update_delta as f32);
        }

        if pressed_keys.contains(&KeyCode::C) {
            self.world.cam.move_down(update_delta as f32);
        }
    }
}

// mouse and keyboard input
impl Stage {
    pub fn key_down_event(&mut self, _keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        // put all the pressed keys in a set so we can check em later if they are pressed down or
        // not, remove them from the set when they are released
        match _keycode {
            KeyCode::Escape => {
                for texture in self.textures.iter_mut() {
                    texture.delete_texture(&mut self.ctx);
                }
                window::request_quit();
            }

            KeyCode::Key1 => {
                self.settings.render_wireframe = !self.settings.render_wireframe;
                self.settings.debug_toggle_1 = !self.settings.debug_toggle_1;
                println!("Toggle wireframe {}", self.settings.render_wireframe);
            }

            KeyCode::Key2 => {
                self.settings.debug_toggle_2 = !self.settings.debug_toggle_2;
                println!("toggled debug 2");
            }

            KeyCode::Key3 => {
                self.settings.debug_toggle_3 = !self.settings.debug_toggle_3;
                println!("toggled debug 3");
            }

            KeyCode::Key4 => {
                self.settings.debug_toggle_4 = !self.settings.debug_toggle_4;
                println!("toggled debug 4");
            }

            _ => {
                self.input.pressed_keys.insert(_keycode);
            }
        }
    }

    pub fn key_up_event(&mut self, _keycode: KeyCode, _keymods: KeyMods) {
        self.input.pressed_keys.remove(&_keycode);
    }

    pub fn mouse_button_down_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {
        self.input.pressed_mouse_buttons.insert(_button);
    }

    pub fn mouse_button_up_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {
        self.input.pressed_mouse_buttons.remove(&_button);
    }

    pub fn mouse_motion_event(&mut self, _x: f32, _y: f32) {
        if !self
            .input
            .pressed_mouse_buttons
            .contains(&MouseButton::Left)
        {
            // nothing to do per se
            self.input.prev_mouse_location = glam::Vec2 { x: _x, y: -_y };
            return;
        }

        // inverse y since the mouse position is top-left 0.0
        // but we want it in "screen space" which means 0.0 is bottom left
        let new_position = glam::Vec2 { x: _x, y: -_y };

        let delta =
            (new_position - self.input.prev_mouse_location) * self.settings.mouse_sensitivity;

        self.input.prev_mouse_location = new_position;

        if delta.x == 0.0 && delta.y == 0.0 {
            return;
        }

        self.world.cam.change_pitch_yaw(delta.x, delta.y);
    }
}

pub mod input {
    use miniquad::{KeyCode, MouseButton};
    use std::collections::HashSet;

    pub struct InputData {
        pub pressed_keys: HashSet<KeyCode>,
        pub pressed_mouse_buttons: HashSet<MouseButton>,

        pub prev_mouse_location: glam::Vec2,
    }

    impl InputData {
        pub fn new() -> Self {
            Self {
                pressed_keys: HashSet::new(),
                pressed_mouse_buttons: HashSet::new(),
                prev_mouse_location: glam::Vec2 { x: 0.0, y: 0.0 },
            }
        }
    }
}
