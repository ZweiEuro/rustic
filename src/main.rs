use miniquad::{gl::{GL_FILL, GL_FRONT_AND_BACK, GL_LINE}, *};
use sdl3::libc::printf;
use std::{panic, sync::Mutex, time::{Duration, SystemTime}};


mod shaders;
mod textures;

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
    uv: Vec2,
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

    textures: Vec<textures::Texture>,
    shaders: Vec<shaders::Shader>,
}

impl Stage {
    pub fn new() -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -0.5, y: -0.5 }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x:  0.5, y: -0.5 }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x:  0.5, y:  0.5 }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x: -0.5, y:  0.5 }, uv: Vec2 { x: 0., y: 1. } },
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
            .new_shader(
                myshader.get_shadersource(),
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("uv_pos", VertexFormat::Float2),
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
            settings,
            textures: vec![texture],
            shaders: vec![myshader],
        }
    }
}




impl EventHandler for Stage {
    fn update(&mut self) {
        for  shader in  self.shaders.iter_mut(){
            if shader.reload_if_needed() {
               // println!("reload!");
            }
        }
    }

    fn key_down_event(&mut self, _keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {

        match _keycode {
            KeyCode::Escape => {
                for texture in self.textures.iter_mut(){
                    texture.delete_texture(&mut self.ctx);
                }
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


        self.ctx.draw(0, 6, 1);
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
                uniforms: vec![],
            },
        }
    }
    #[repr(C)]
    pub struct Uniforms {
    }
}


