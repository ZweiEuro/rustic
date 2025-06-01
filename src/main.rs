use std::{
    collections::HashSet,
    sync::{LazyLock, Mutex},
};

use glm::{Vec2, Vec3, radians};
use miniquad::{
    gl::{GL_DEPTH_BUFFER_BIT, GL_FILL, GL_FRONT_AND_BACK, GL_LINE, GL_TRIANGLES},
    *,
};
use stage::Camera;

/**
* General Notes:
* - Not sure if mipmaps work correctly
*/
mod shaders;
mod stage;
mod textures;

#[rustfmt::skip]
const M4_UNIT: glm::Mat4 =  glm::Mat4 { 
    c0: glm::Vec4{ x: 1.0, y: 0.0, z: 0.0, w: 0.0},
    c1: glm::Vec4{ x: 0.0, y: 1.0, z: 0.0, w: 0.0},
    c2: glm::Vec4{ x: 0.0, y: 0.0, z: 1.0, w: 0.0},
    c3: glm::Vec4{ x: 0.0, y: 0.0, z: 0.0, w: 1.0},
};

#[repr(C)]
struct Vertex {
    pos: Vec3,
    uv: Vec2,
}

struct Settings {
    render_wireframe: bool,
    debug_toggle_1: bool,
    debug_toggle_2: bool,
    debug_toggle_3: bool,
    debug_toggle_4: bool,
}

const CAMERA_SPEED: f32 = 0.05;
const MOUSE_SENSITIVITY: f32 = 0.2;

struct WorldState {
    cam: Camera,

    model: glm::Mat4,
    view: glm::Mat4,
    projection: glm::Mat4,
}

struct Input {
    pressed_keys: HashSet<KeyCode>,
    pressed_mouse_buttons: HashSet<MouseButton>,

    prev_mouse_location: Vec2,
}

struct Stage {
    ctx: Box<dyn RenderingBackend>,

    world: WorldState,

    pipeline: Pipeline,
    bindings: Bindings,

    settings: Settings,

    textures: Vec<textures::Texture>,
    shaders: Vec<shaders::Shader>,

    input: Input,
}

impl Stage {
    pub fn new() -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        #[rustfmt::skip]
        let vertices: [Vertex; 36] = [
            Vertex { pos: Vec3 { x: -0.5, y: -0.5, z: -0.5}, uv: Vec2 { x: 0.0, y: 0.0}},
            Vertex { pos: Vec3 { x:  0.5, y: -0.5, z: -0.5}, uv: Vec2 { x: 1.0, y: 0.0}},
            Vertex { pos: Vec3 { x:  0.5, y:  0.5, z: -0.5}, uv: Vec2 { x: 1.0, y: 1.0}},
            Vertex { pos: Vec3 { x:  0.5, y:  0.5, z: -0.5}, uv: Vec2 { x: 1.0, y: 1.0}},
            Vertex { pos: Vec3 { x: -0.5, y:  0.5, z: -0.5}, uv: Vec2 { x: 0.0, y: 1.0}},
            Vertex { pos: Vec3 { x: -0.5, y: -0.5, z: -0.5}, uv: Vec2 { x: 0.0, y: 0.0}},
            Vertex { pos: Vec3 { x: -0.5, y: -0.5, z:  0.5}, uv: Vec2 { x: 0.0, y: 0.0}},
            Vertex { pos: Vec3 { x:  0.5, y: -0.5, z:  0.5}, uv: Vec2 { x: 1.0, y: 0.0}},
            Vertex { pos: Vec3 { x:  0.5, y:  0.5, z:  0.5}, uv: Vec2 { x: 1.0, y: 1.0}},
            Vertex { pos: Vec3 { x:  0.5, y:  0.5, z:  0.5}, uv: Vec2 { x: 1.0, y: 1.0}},
            Vertex { pos: Vec3 { x: -0.5, y:  0.5, z:  0.5}, uv: Vec2 { x: 0.0, y: 1.0}},
            Vertex { pos: Vec3 { x: -0.5, y: -0.5, z:  0.5}, uv: Vec2 { x: 0.0, y: 0.0}},
            Vertex { pos: Vec3 { x: -0.5, y:  0.5, z:  0.5}, uv: Vec2 { x: 1.0, y: 0.0}},
            Vertex { pos: Vec3 { x: -0.5, y:  0.5, z: -0.5}, uv: Vec2 { x: 1.0, y: 1.0}},
            Vertex { pos: Vec3 { x: -0.5, y: -0.5, z: -0.5}, uv: Vec2 { x: 0.0, y: 1.0}},
            Vertex { pos: Vec3 { x: -0.5, y: -0.5, z: -0.5}, uv: Vec2 { x: 0.0, y: 1.0}},
            Vertex { pos: Vec3 { x: -0.5, y: -0.5, z:  0.5}, uv: Vec2 { x: 0.0, y: 0.0}},
            Vertex { pos: Vec3 { x: -0.5, y:  0.5, z:  0.5}, uv: Vec2 { x: 1.0, y: 0.0}},
            Vertex { pos: Vec3 { x:  0.5, y:  0.5, z:  0.5}, uv: Vec2 { x: 1.0, y: 0.0}},
            Vertex { pos: Vec3 { x:  0.5, y:  0.5, z: -0.5}, uv: Vec2 { x: 1.0, y: 1.0}},
            Vertex { pos: Vec3 { x:  0.5, y: -0.5, z: -0.5}, uv: Vec2 { x: 0.0, y: 1.0}},
            Vertex { pos: Vec3 { x:  0.5, y: -0.5, z: -0.5}, uv: Vec2 { x: 0.0, y: 1.0}},
            Vertex { pos: Vec3 { x:  0.5, y: -0.5, z:  0.5}, uv: Vec2 { x: 0.0, y: 0.0}},
            Vertex { pos: Vec3 { x:  0.5, y:  0.5, z:  0.5}, uv: Vec2 { x: 1.0, y: 0.0}},
            Vertex { pos: Vec3 { x: -0.5, y: -0.5, z: -0.5}, uv: Vec2 { x: 0.0, y: 1.0}},
            Vertex { pos: Vec3 { x:  0.5, y: -0.5, z: -0.5}, uv: Vec2 { x: 1.0, y: 1.0}},
            Vertex { pos: Vec3 { x:  0.5, y: -0.5, z:  0.5}, uv: Vec2 { x: 1.0, y: 0.0}},
            Vertex { pos: Vec3 { x:  0.5, y: -0.5, z:  0.5}, uv: Vec2 { x: 1.0, y: 0.0}},
            Vertex { pos: Vec3 { x: -0.5, y: -0.5, z:  0.5}, uv: Vec2 { x: 0.0, y: 0.0}},
            Vertex { pos: Vec3 { x: -0.5, y: -0.5, z: -0.5}, uv: Vec2 { x: 0.0, y: 1.0}},
            Vertex { pos: Vec3 { x: -0.5, y:  0.5, z: -0.5}, uv: Vec2 { x: 0.0, y: 1.0}},
            Vertex { pos: Vec3 { x:  0.5, y:  0.5, z: -0.5}, uv: Vec2 { x: 1.0, y: 1.0}},
            Vertex { pos: Vec3 { x:  0.5, y:  0.5, z:  0.5}, uv: Vec2 { x: 1.0, y: 0.0}},
            Vertex { pos: Vec3 { x:  0.5, y:  0.5, z:  0.5}, uv: Vec2 { x: 1.0, y: 0.0}},
            Vertex { pos: Vec3 { x: -0.5, y:  0.5, z:  0.5}, uv: Vec2 { x: 0.0, y: 0.0}},
            Vertex { pos: Vec3 { x: -0.5, y:  0.5, z: -0.5}, uv: Vec2 { x: 0.0, y: 1.0}},
        ];

        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let indices: [u16; 6] = [0, 1, 3, 1, 2, 3];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let mut texture = textures::Texture::new("test.png".to_owned());

        let texture_id = texture.get_texture_id(&mut ctx);
        ctx.texture_set_filter(texture_id, FilterMode::Nearest, MipmapFilterMode::None);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![texture_id],
        };

        let myshader = shaders::Shader::new("basic".to_owned());

        let shader = ctx
            .new_shader(myshader.get_shadersource(), shader::meta())
            .unwrap();

        let mut pipelineparams = PipelineParams::default();
        pipelineparams.depth_test = Comparison::Less;
        pipelineparams.depth_write = true;

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float3),
                VertexAttribute::new("uv_pos", VertexFormat::Float2),
            ],
            shader,
            pipelineparams,
        );

        let settings = Settings {
            render_wireframe: false,
            debug_toggle_1: false,
            debug_toggle_2: false,
            debug_toggle_3: false,
            debug_toggle_4: false,
        };

        let perspective = glm::ext::perspective(glm::radians(45.0), 1.0, 0.1, 100.0);

        Stage {
            pipeline,
            bindings,
            ctx,
            settings,
            textures: vec![texture],
            shaders: vec![myshader],
            input: Input {
                pressed_keys: HashSet::new(),
                pressed_mouse_buttons: HashSet::new(),
                prev_mouse_location: Vec2 { x: 0.0, y: 0.0 },
            },
            world: WorldState {
                cam: Camera {
                    camera_pos: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 10.0,
                    },
                    camera_front: Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: -1.0,
                    },
                    camera_up: Vec3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },

                    pitch: 0.0,
                    yaw: -90.0,
                },
                model: M4_UNIT.clone(),
                view: M4_UNIT.clone(),
                projection: perspective,
            },
        }
    }
}

static LAST_TIME_UPDATED: Mutex<f64> = Mutex::new(0.0);
static TOTAL_TIME: Mutex<f64> = Mutex::new(0.0);

impl EventHandler for Stage {
    fn update(&mut self) {
        for shader in self.shaders.iter_mut() {
            if shader.reload_if_needed() {
                // println!("reload!");
            }
        }

        let mut time = LAST_TIME_UPDATED.lock().unwrap();
        let mut total_time = TOTAL_TIME.lock().unwrap();

        if *time == 0.0 {
            *time = date::now();
        }

        let delta = date::now() - *time;

        let pressed_keys = self.input.pressed_keys.clone();

        // forward and back
        if pressed_keys.contains(&KeyCode::W) {
            self.world.cam.move_forward(CAMERA_SPEED);
        }

        if pressed_keys.contains(&KeyCode::S) {
            self.world.cam.move_backwards(CAMERA_SPEED);
        }

        // left and right
        if pressed_keys.contains(&KeyCode::A) {
            self.world.cam.move_left(CAMERA_SPEED);
        }

        if pressed_keys.contains(&KeyCode::D) {
            self.world.cam.move_right(CAMERA_SPEED);
        }

        // up and down
        if pressed_keys.contains(&KeyCode::Space) {
            self.world.cam.move_up(CAMERA_SPEED);
        }

        if pressed_keys.contains(&KeyCode::LeftShift) {
            self.world.cam.move_down(CAMERA_SPEED);
        }

        *time = date::now();
        *total_time += delta;
    }

    fn key_down_event(&mut self, _keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
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

    fn key_up_event(&mut self, _keycode: KeyCode, _keymods: KeyMods) {
        self.input.pressed_keys.remove(&_keycode);
    }

    fn mouse_button_down_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {
        self.input.pressed_mouse_buttons.insert(_button);
        println!("button down at {:?}", self.input.prev_mouse_location);
    }

    fn mouse_button_up_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {
        self.input.pressed_mouse_buttons.remove(&_button);
    }

    fn mouse_motion_event(&mut self, _x: f32, _y: f32) {
        if !self
            .input
            .pressed_mouse_buttons
            .contains(&MouseButton::Left)
        {
            // nothing to do per se
            self.input.prev_mouse_location = Vec2 { x: _x, y: -_y };
            return;
        }

        // inverse y since the mouse position is top-left 0.0
        // but we want it in "screen space" which means 0.0 is bottom left
        let new_position = Vec2 { x: _x, y: -_y };

        let delta = (new_position - self.input.prev_mouse_location) * MOUSE_SENSITIVITY;

        self.input.prev_mouse_location = new_position;

        if delta.x == 0.0 && delta.y == 0.0 {
            return;
        }

        self.world.cam.yaw += delta.x;
        self.world.cam.pitch += delta.y;

        self.world.cam.pitch = self.world.cam.pitch.clamp(-89.0, 89.0);

        let direction = Vec3 {
            x: glm::cos(radians(self.world.cam.yaw)) * glm::cos(glm::radians(self.world.cam.pitch)),
            y: glm::sin(glm::radians(self.world.cam.pitch)),
            z: glm::sin(radians(self.world.cam.yaw)) * glm::cos(glm::radians(self.world.cam.pitch)),
        };

        self.world.cam.camera_front = glm::normalize(direction);
    }

    fn draw(&mut self) {
        self.ctx.begin_default_pass(Default::default());

        unsafe {
            gl::glEnable(GL_DEPTH_BUFFER_BIT);
        }

        self.ctx.clear(Some((0.0, 0.0, 0.0, 0.0)), Some(1.0), None);

        unsafe {
            // toggle the wireframe rendering by changing the gl polygon format
            if self.settings.render_wireframe {
                raw_gl::glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);
            } else {
                raw_gl::glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);
            }
        }

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);

        #[rustfmt::skip]
        let cube_pos: [Vec3; 10] = [
            Vec3 { x:  0.0, y:  0.0,z:  0.0 },
            Vec3 { x:  2.0, y:  5.0,z: -15.0 },
            Vec3 { x: -1.5, y: -2.2,z: -2.5 },
            Vec3 { x: -3.8, y: -2.0,z: -12.3 },
            Vec3 { x:  2.4, y: -0.4,z: -3.5 },
            Vec3 { x: -1.7, y:  3.0,z: -7.5 },
            Vec3 { x:  1.3, y: -2.0,z: -2.5 },
            Vec3 { x:  1.5, y:  2.0,z: -2.5 },
            Vec3 { x:  1.5, y:  0.2,z: -1.5 },
            Vec3 { x: -1.3, y:  1.0,z: -1.5 },
        ];

        let view = glm::ext::look_at(
            self.world.cam.camera_pos,
            self.world.cam.camera_pos + self.world.cam.camera_front,
            self.world.cam.camera_up,
        );

        for pos in cube_pos.iter() {
            self.ctx
                .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                    model: glm::ext::translate(&M4_UNIT, *pos).clone(),
                    view: view,
                    projection: self.world.projection,
                }));

            unsafe {
                gl::glDrawArrays(GL_TRIANGLES, 0, 36);
            }
        }

        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}

fn main() {
    println!("Hello world");

    let mut conf = conf::Conf::default();
    conf.platform.apple_gfx_api = conf::AppleGfxApi::OpenGl;
    conf.window_height = 600;
    conf.window_width = 600;

    miniquad::start(conf, move || Box::new(Stage::new()));

    println!("exiting miniquad");
}

mod shader {
    use glm::Mat4;
    use miniquad::*;
    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("model", UniformType::Mat4),
                    UniformDesc::new("view", UniformType::Mat4),
                    UniformDesc::new("projection", UniformType::Mat4),
                ],
            },
        }
    }
    #[repr(C)]
    pub struct Uniforms {
        pub model: Mat4,
        pub view: Mat4,
        pub projection: Mat4,
    }
}
