use std::sync::Mutex;

use glm::{Vec3, Vec2};
use miniquad::{gl::{GL_DEPTH_BUFFER_BIT, GL_FILL, GL_FRONT_AND_BACK, GL_LINE, GL_TRIANGLES}, *};


/**
* General Notes:
* - Not sure if mipmaps work correctly
*/

mod shaders;
mod textures;

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

struct WorldState {

    model: glm::Mat4,
    view: glm::Mat4,
    projection: glm::Mat4,

}

struct Stage {
    ctx: Box<dyn RenderingBackend>,

    world: WorldState,

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
            .new_shader(
                myshader.get_shadersource(),
                shader::meta(),
            )
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
            pipelineparams
        );


        let settings = Settings {
            render_wireframe: false,
            debug_toggle_1 : false,
            debug_toggle_2 : false,
            debug_toggle_3 : false,
            debug_toggle_4 : false,
        };

        let view = glm::ext::look_at(
            Vec3 {x: 0.0, y: 0.0, z: 3.0 }, 
            Vec3 {x: 0.0, y: 0.0, z: 0.0 },
            Vec3 {x: 0.0, y: 1.0, z: 0.0 }
        );

        Stage {
            pipeline,
            bindings,
            ctx,
            settings,
            textures: vec![texture],
            shaders: vec![myshader],
            world: WorldState {
                model: M4_UNIT.clone(), 
                view: M4_UNIT.clone(), 
                projection: M4_UNIT.clone(), 
            }
        }
    }
}



static LAST_TIME_UPDATED: Mutex<f64> = Mutex::new(0.0);
static TOTAL_TIME: Mutex<f64> = Mutex::new(0.0);

impl EventHandler for Stage {
    fn update(&mut self) {
        for  shader in  self.shaders.iter_mut(){
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


        self.world.model = glm::ext::rotate(&self.world.model, delta as f32 * glm::radians(50.0), Vec3 {x: 0.5, y: 1.0, z: 0.0});

        *time = date::now();
        *total_time += delta;




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
                self.world.model = glm::ext::translate(
                    &self.world.model, Vec3 { x: 0.0, y: 0.1, z: 0.0 }
                );
            }

            KeyCode::A => {           
                self.world.model = glm::ext::translate(
                    &self.world.model, Vec3 { x: -0.1, y: 0.0, z: 0.0 }
                );
            }

            KeyCode::S => {           
                self.world.model = glm::ext::translate(
                    &self.world.model, Vec3 { x: 0.0, y: -0.1, z: 0.0 }
                );
            }

            KeyCode::D => {           
                self.world.model = glm::ext::translate(
                    &self.world.model, Vec3 { x: 0.1, y: 0.0, z: 0.0 }
                );
            }

            KeyCode::Q => {
                self.world.view = glm::ext::rotate(
                    &self.world.view,
                    glm::builtin::radians(10.0),
                    glm::vec3(0.0, 1.0, 0.0)
                );
            }

            KeyCode::E => {
                self.world.view = glm::ext::rotate(
                    &self.world.view,
                    glm::builtin::radians(-10.0),
                    glm::vec3(0.0, 1.0, 0.0)
                );
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
                println!("Unhandled key down event {}", _keycode as u16);
            }  
        }

    }

    fn draw(&mut self) {

        self.ctx.begin_default_pass(Default::default());

        unsafe {
            gl::glEnable(GL_DEPTH_BUFFER_BIT);
        }

        self.ctx.clear(Some((0.0,0.0,0.0,0.0)), Some(1.0), None);


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

        self.ctx
            .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                model: self.world.model,
                view: self.world.view,
                projection: self.world.projection,
            }));

        unsafe {
            gl::glDrawArrays(GL_TRIANGLES, 0, 36);
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


