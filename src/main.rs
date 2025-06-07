use miniquad::{
    gl::{GL_DEPTH_BUFFER_BIT, GL_FILL, GL_FRONT_AND_BACK, GL_LINE, GL_TRIANGLES},
    *,
};
use stage::{input::InputData, *};

/**
* General Notes:
* - Not sure if mipmaps work correctly
*/
mod shaders;
mod stage;
mod textures;

#[repr(C)]
struct Vertex {
    pos: glam::Vec3,
    uv: glam::Vec2,
}

impl Stage {
    pub fn new() -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        miniquad::window::set_cursor_grab(true);
        #[rustfmt::skip]
        let vertices: [Vertex; 36] = [
            Vertex { pos: glam::vec3(-0.5, -0.5, -0.5), uv: glam::vec2(0.0, 0.0)},
            Vertex { pos: glam::vec3( 0.5, -0.5, -0.5), uv: glam::vec2(1.0, 0.0)},
            Vertex { pos: glam::vec3( 0.5,  0.5, -0.5), uv: glam::vec2(1.0, 1.0)},
            Vertex { pos: glam::vec3( 0.5,  0.5, -0.5), uv: glam::vec2(1.0, 1.0)},
            Vertex { pos: glam::vec3(-0.5,  0.5, -0.5), uv: glam::vec2(0.0, 1.0)},
            Vertex { pos: glam::vec3(-0.5, -0.5, -0.5), uv: glam::vec2(0.0, 0.0)},
            Vertex { pos: glam::vec3(-0.5, -0.5,  0.5), uv: glam::vec2(0.0, 0.0)},
            Vertex { pos: glam::vec3( 0.5, -0.5,  0.5), uv: glam::vec2(1.0, 0.0)},
            Vertex { pos: glam::vec3( 0.5,  0.5,  0.5), uv: glam::vec2(1.0, 1.0)},
            Vertex { pos: glam::vec3( 0.5,  0.5,  0.5), uv: glam::vec2(1.0, 1.0)},
            Vertex { pos: glam::vec3(-0.5,  0.5,  0.5), uv: glam::vec2(0.0, 1.0)},
            Vertex { pos: glam::vec3(-0.5, -0.5,  0.5), uv: glam::vec2(0.0, 0.0)},
            Vertex { pos: glam::vec3(-0.5,  0.5,  0.5), uv: glam::vec2(1.0, 0.0)},
            Vertex { pos: glam::vec3(-0.5,  0.5, -0.5), uv: glam::vec2(1.0, 1.0)},
            Vertex { pos: glam::vec3(-0.5, -0.5, -0.5), uv: glam::vec2(0.0, 1.0)},
            Vertex { pos: glam::vec3(-0.5, -0.5, -0.5), uv: glam::vec2(0.0, 1.0)},
            Vertex { pos: glam::vec3(-0.5, -0.5,  0.5), uv: glam::vec2(0.0, 0.0)},
            Vertex { pos: glam::vec3(-0.5,  0.5,  0.5), uv: glam::vec2(1.0, 0.0)},
            Vertex { pos: glam::vec3( 0.5,  0.5,  0.5), uv: glam::vec2(1.0, 0.0)},
            Vertex { pos: glam::vec3( 0.5,  0.5, -0.5), uv: glam::vec2(1.0, 1.0)},
            Vertex { pos: glam::vec3( 0.5, -0.5, -0.5), uv: glam::vec2(0.0, 1.0)},
            Vertex { pos: glam::vec3( 0.5, -0.5, -0.5), uv: glam::vec2(0.0, 1.0)},
            Vertex { pos: glam::vec3( 0.5, -0.5,  0.5), uv: glam::vec2(0.0, 0.0)},
            Vertex { pos: glam::vec3( 0.5,  0.5,  0.5), uv: glam::vec2(1.0, 0.0)},
            Vertex { pos: glam::vec3(-0.5, -0.5, -0.5), uv: glam::vec2(0.0, 1.0)},
            Vertex { pos: glam::vec3( 0.5, -0.5, -0.5), uv: glam::vec2(1.0, 1.0)},
            Vertex { pos: glam::vec3( 0.5, -0.5,  0.5), uv: glam::vec2(1.0, 0.0)},
            Vertex { pos: glam::vec3( 0.5, -0.5,  0.5), uv: glam::vec2(1.0, 0.0)},
            Vertex { pos: glam::vec3(-0.5, -0.5,  0.5), uv: glam::vec2(0.0, 0.0)},
            Vertex { pos: glam::vec3(-0.5, -0.5, -0.5), uv: glam::vec2(0.0, 1.0)},
            Vertex { pos: glam::vec3(-0.5,  0.5, -0.5), uv: glam::vec2(0.0, 1.0)},
            Vertex { pos: glam::vec3( 0.5,  0.5, -0.5), uv: glam::vec2(1.0, 1.0)},
            Vertex { pos: glam::vec3( 0.5,  0.5,  0.5), uv: glam::vec2(1.0, 0.0)},
            Vertex { pos: glam::vec3( 0.5,  0.5,  0.5), uv: glam::vec2(1.0, 0.0)},
            Vertex { pos: glam::vec3(-0.5,  0.5,  0.5), uv: glam::vec2(0.0, 0.0)},
            Vertex { pos: glam::vec3(-0.5,  0.5, -0.5), uv: glam::vec2(0.0, 1.0)},
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

        let myshader = shaders::ShaderFile::new("basic".to_owned());

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
            mouse_sensitivity: 0.2,

            render_wireframe: false,
            debug_toggle_1: false,
            debug_toggle_2: false,
            debug_toggle_3: false,
            debug_toggle_4: false,
        };

        Stage {
            pipeline,
            bindings,
            ctx,
            settings,
            textures: vec![texture],
            shaders: vec![myshader],
            input: InputData::new(),
            world: WorldState {
                cam: Camera {
                    camera_pos: glam::Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: 10.0,
                    },
                    camera_front: glam::Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: -1.0,
                    },
                    camera_up: glam::Vec3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },

                    pitch: 0.0,
                    yaw: -90.0,
                    camera_speed: 5.2,

                    // perspective
                    fov_y_deg: 45.0,
                    aspect_ratio: 1.0,
                    z_near: 0.1,
                    z_far: 100.0,
                },
            },
            meta: StageMetadata {
                last_time_update_fn_run: date::now(),
                _time_stage_started: date::now(),
            },
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {
        self.update();
    }

    fn key_down_event(&mut self, _keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        self.key_down_event(_keycode, _keymods, _repeat);
    }

    fn key_up_event(&mut self, _keycode: KeyCode, _keymods: KeyMods) {
        self.key_up_event(_keycode, _keymods);
    }

    fn mouse_button_down_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {
        self.mouse_button_down_event(_button, _x, _y);
    }

    fn mouse_button_up_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {
        self.mouse_button_up_event(_button, _x, _y);
    }

    fn mouse_motion_event(&mut self, _x: f32, _y: f32) {
        self.mouse_motion_event(_x, _y);
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
        let cube_pos: [glam::Vec3; 10] = [
            glam::Vec3 { x:  0.0, y:  0.0,z:  0.0 },
            glam::Vec3 { x:  2.0, y:  5.0,z: -15.0 },
            glam::Vec3 { x: -1.5, y: -2.2,z: -2.5 },
            glam::Vec3 { x: -3.8, y: -2.0,z: -12.3 },
            glam::Vec3 { x:  2.4, y: -0.4,z: -3.5 },
            glam::Vec3 { x: -1.7, y:  3.0,z: -7.5 },
            glam::Vec3 { x:  1.3, y: -2.0,z: -2.5 },
            glam::Vec3 { x:  1.5, y:  2.0,z: -2.5 },
            glam::Vec3 { x:  1.5, y:  0.2,z: -1.5 },
            glam::Vec3 { x: -1.3, y:  1.0,z: -1.5 },
        ];

        for pos in cube_pos.iter() {
            self.ctx
                .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                    model: glam::Mat4::from_translation(*pos),
                    view: self.world.cam.get_view_matrix(),
                    projection: self.world.cam.get_perspective_matrix(),
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
        pub model: glam::f32::Mat4,
        pub view: glam::f32::Mat4,
        pub projection: glam::f32::Mat4,
    }
}
