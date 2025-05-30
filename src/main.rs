use miniquad::{gl::{GL_FILL, GL_FRONT_AND_BACK, GL_LINE}, *};
use std::{panic, sync::Mutex, time::{Duration, SystemTime}};


mod shaders;


#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}

#[repr(C)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec2,
    color: Vec3,
}

struct Settings {
    render_wireframe: bool,
    debug_toggle_1: bool,
    debug_toggle_2: bool,
    debug_toggle_3: bool,
    debug_toggle_4: bool,
}

struct Stage {
    ctx: Box<dyn RenderingBackend>,

    pipeline: Pipeline,
    bindings: Bindings,

    settings: Settings,

}

impl Stage {
    pub fn new() -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x:  0.5, y:  0.5}, color: Vec3{ x: 1.0, y: 0.0, z: 0.0}},
            Vertex { pos : Vec2 { x:  0.5, y: -0.5}, color: Vec3{ x: 0.0, y: 1.0, z: 0.0}},
            Vertex { pos : Vec2 { x: -0.5, y: -0.5}, color: Vec3{ x: 0.0, y: 0.0, z: 1.0}},
            Vertex { pos : Vec2 { x: -0.5, y:  0.5}, color: Vec3{ x: 0.0, y: 0.0, z: 0.0}},
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


        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };

        let shader = ctx
            .new_shader(
                shaders::Shader::new("basic".to_owned()).get_shadersource(),
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("aColor", VertexFormat::Float3),
            ],
            shader,
            PipelineParams::default(),
        );


        let settings = Settings {
            render_wireframe: false,
            debug_toggle_1 : false,
            debug_toggle_2 : false,
            debug_toggle_3 : false,
            debug_toggle_4 : false,
        };


        Stage {
            pipeline,
            bindings,
            ctx,
            settings
        }
    }
}




impl EventHandler for Stage {
    fn update(&mut self) {}

    fn key_down_event(&mut self, _keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {

        match _keycode {
            KeyCode::Escape => {
                window::request_quit();
            }
            KeyCode::W => {
                self.settings.render_wireframe = !self.settings.render_wireframe;
                println!("Toggle wireframe {}", self.settings.render_wireframe);
            }

            KeyCode::Key1 => {
                self.settings.debug_toggle_1 = !self.settings.debug_toggle_1;
                println!("toggled debug 1");
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
                println!("Unhandled key down event {}", _keycode as u16);
            }  
        }

    }

    fn draw(&mut self) {

        self.ctx.begin_default_pass(Default::default());

        self.ctx.clear(Some((0.0,0.0,0.0,0.0)), None, None);


        unsafe{
            // toggle the wireframe rendering by changing the gl polygon format
            if self.settings.render_wireframe {
                raw_gl::glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);
            }else {
                raw_gl::glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);
            }
        }

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);


        let t = date::now();
        if self.settings.debug_toggle_1 {
            self.ctx
                .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                    our_color: (t.sin() as f32 + 1.0, 0.0, 0.0, 1.0),
                }));

        } else{

            self.ctx
                .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                    our_color: (0.0, t.sin() as f32 + 1.0, 0.0, 1.0),
                }));
        }


        self.ctx.draw(0, 6, 1);
        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}

fn main() {

    println!("Hello world");



    let mut conf = conf::Conf::default();

    conf.platform.apple_gfx_api = conf::AppleGfxApi::OpenGl;

    miniquad::start(conf, move || Box::new(Stage::new()));
}

mod shader {
    use miniquad::*;
    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("ourColor", UniformType::Float4)],
            },
        }
    }
    #[repr(C)]
    pub struct Uniforms {
        pub our_color: (f32, f32, f32, f32),
    }
}


