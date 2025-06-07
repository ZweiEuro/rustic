use std::panic;

use miniquad::{
    Bindings, BufferId, BufferSource, BufferType, BufferUsage, FilterMode, MipmapFilterMode,
    RenderingBackend, TextureId,
};

use crate::textures;

pub struct DataVertex3DTexture {
    pub pos: glam::Vec3,
    pub uv: glam::Vec2,
}

pub struct DataVertex3D {
    pos: glam::Vec3,
}

type BackendArg = Box<dyn RenderingBackend>;

/// an object that can be rendered by opengl with the appropriate pipeline
pub trait RenderableObject {
    /// get the binding, buffered if possible
    /// Should any resource be missing (opengl, texture, etc.) create
    fn get_bindings(self: &mut Self, ctx: &mut BackendArg) -> Bindings;

    /// security check that everything was deleted correctly
    fn drop(self: &mut Self);

    /// deallocate any resources that are allocated on opengl
    fn drop_gl_resources(self: &mut Self, ctx: &mut BackendArg);
}

pub struct TestTexturedCube {
    pub vertices: [DataVertex3DTexture; 36],

    /// opengl vars

    ///
    vertex_buffer_id: Option<BufferId>,
    index_buffer_id: Option<BufferId>,
    texture: Option<textures::Texture>,
    texture_id: Option<TextureId>,
}

impl TestTexturedCube {
    pub fn new() -> Self {
        TestTexturedCube {
            #[rustfmt::skip]
            vertices: [
                DataVertex3DTexture { pos: glam::vec3(-0.5, -0.5, -0.5), uv: glam::vec2(0.0, 0.0)},
                DataVertex3DTexture { pos: glam::vec3( 0.5, -0.5, -0.5), uv: glam::vec2(1.0, 0.0)},
                DataVertex3DTexture { pos: glam::vec3( 0.5,  0.5, -0.5), uv: glam::vec2(1.0, 1.0)},
                DataVertex3DTexture { pos: glam::vec3( 0.5,  0.5, -0.5), uv: glam::vec2(1.0, 1.0)},
                DataVertex3DTexture { pos: glam::vec3(-0.5,  0.5, -0.5), uv: glam::vec2(0.0, 1.0)},
                DataVertex3DTexture { pos: glam::vec3(-0.5, -0.5, -0.5), uv: glam::vec2(0.0, 0.0)},
                DataVertex3DTexture { pos: glam::vec3(-0.5, -0.5,  0.5), uv: glam::vec2(0.0, 0.0)},
                DataVertex3DTexture { pos: glam::vec3( 0.5, -0.5,  0.5), uv: glam::vec2(1.0, 0.0)},
                DataVertex3DTexture { pos: glam::vec3( 0.5,  0.5,  0.5), uv: glam::vec2(1.0, 1.0)},
                DataVertex3DTexture { pos: glam::vec3( 0.5,  0.5,  0.5), uv: glam::vec2(1.0, 1.0)},
                DataVertex3DTexture { pos: glam::vec3(-0.5,  0.5,  0.5), uv: glam::vec2(0.0, 1.0)},
                DataVertex3DTexture { pos: glam::vec3(-0.5, -0.5,  0.5), uv: glam::vec2(0.0, 0.0)},
                DataVertex3DTexture { pos: glam::vec3(-0.5,  0.5,  0.5), uv: glam::vec2(1.0, 0.0)},
                DataVertex3DTexture { pos: glam::vec3(-0.5,  0.5, -0.5), uv: glam::vec2(1.0, 1.0)},
                DataVertex3DTexture { pos: glam::vec3(-0.5, -0.5, -0.5), uv: glam::vec2(0.0, 1.0)},
                DataVertex3DTexture { pos: glam::vec3(-0.5, -0.5, -0.5), uv: glam::vec2(0.0, 1.0)},
                DataVertex3DTexture { pos: glam::vec3(-0.5, -0.5,  0.5), uv: glam::vec2(0.0, 0.0)},
                DataVertex3DTexture { pos: glam::vec3(-0.5,  0.5,  0.5), uv: glam::vec2(1.0, 0.0)},
                DataVertex3DTexture { pos: glam::vec3( 0.5,  0.5,  0.5), uv: glam::vec2(1.0, 0.0)},
                DataVertex3DTexture { pos: glam::vec3( 0.5,  0.5, -0.5), uv: glam::vec2(1.0, 1.0)},
                DataVertex3DTexture { pos: glam::vec3( 0.5, -0.5, -0.5), uv: glam::vec2(0.0, 1.0)},
                DataVertex3DTexture { pos: glam::vec3( 0.5, -0.5, -0.5), uv: glam::vec2(0.0, 1.0)},
                DataVertex3DTexture { pos: glam::vec3( 0.5, -0.5,  0.5), uv: glam::vec2(0.0, 0.0)},
                DataVertex3DTexture { pos: glam::vec3( 0.5,  0.5,  0.5), uv: glam::vec2(1.0, 0.0)},
                DataVertex3DTexture { pos: glam::vec3(-0.5, -0.5, -0.5), uv: glam::vec2(0.0, 1.0)},
                DataVertex3DTexture { pos: glam::vec3( 0.5, -0.5, -0.5), uv: glam::vec2(1.0, 1.0)},
                DataVertex3DTexture { pos: glam::vec3( 0.5, -0.5,  0.5), uv: glam::vec2(1.0, 0.0)},
                DataVertex3DTexture { pos: glam::vec3( 0.5, -0.5,  0.5), uv: glam::vec2(1.0, 0.0)},
                DataVertex3DTexture { pos: glam::vec3(-0.5, -0.5,  0.5), uv: glam::vec2(0.0, 0.0)},
                DataVertex3DTexture { pos: glam::vec3(-0.5, -0.5, -0.5), uv: glam::vec2(0.0, 1.0)},
                DataVertex3DTexture { pos: glam::vec3(-0.5,  0.5, -0.5), uv: glam::vec2(0.0, 1.0)},
                DataVertex3DTexture { pos: glam::vec3( 0.5,  0.5, -0.5), uv: glam::vec2(1.0, 1.0)},
                DataVertex3DTexture { pos: glam::vec3( 0.5,  0.5,  0.5), uv: glam::vec2(1.0, 0.0)},
                DataVertex3DTexture { pos: glam::vec3( 0.5,  0.5,  0.5), uv: glam::vec2(1.0, 0.0)},
                DataVertex3DTexture { pos: glam::vec3(-0.5,  0.5,  0.5), uv: glam::vec2(0.0, 0.0)},
                DataVertex3DTexture { pos: glam::vec3(-0.5,  0.5, -0.5), uv: glam::vec2(0.0, 1.0)},
            ],
            vertex_buffer_id: None,
            index_buffer_id: None,
            texture: None,
            texture_id: None,
        }
    }
}

impl RenderableObject for TestTexturedCube {
    fn get_bindings(self: &mut Self, ctx: &mut BackendArg) -> Bindings {
        if self.vertex_buffer_id.is_none() {
            self.vertex_buffer_id = Some(ctx.new_buffer(
                BufferType::VertexBuffer,
                BufferUsage::Immutable,
                BufferSource::slice(&self.vertices),
            ));
        }

        if self.index_buffer_id.is_none() {
            let indices: [u16; 6] = [0, 1, 3, 1, 2, 3];
            self.index_buffer_id = Some(ctx.new_buffer(
                BufferType::IndexBuffer,
                BufferUsage::Immutable,
                BufferSource::slice(&indices),
            ));
        }

        if self.texture_id.is_none() {
            let texture = textures::Texture::new("test.png".to_owned());

            self.texture_id = Some(ctx.new_texture_from_rgba8(
                texture.width as u16,
                texture.width as u16,
                unsafe {
                    std::slice::from_raw_parts(
                        texture.img,
                        (texture.width * texture.height * 4) as usize,
                    )
                },
            ));

            ctx.texture_set_filter(
                self.texture_id.clone().unwrap(),
                FilterMode::Nearest,
                MipmapFilterMode::None,
            );
            self.texture = Some(texture);
        }

        return Bindings {
            vertex_buffers: vec![self.vertex_buffer_id.unwrap()],
            index_buffer: self.index_buffer_id.unwrap(),
            images: vec![self.texture_id.clone().unwrap()],
        };
    }

    fn drop_gl_resources(self: &mut Self, ctx: &mut BackendArg) {
        if let Some(val) = self.vertex_buffer_id.take() {
            ctx.delete_buffer(val);
        }

        if let Some(val) = self.index_buffer_id.take() {
            ctx.delete_buffer(val);
        }

        if let Some( val) = self.texture_id.take() {
            ctx.delete_texture(val);
        }
    }

    fn drop(self: &mut Self) {
        if self.vertex_buffer_id.is_some()
            || self.texture.is_some()
            || self.texture_id.is_some()
            || self.index_buffer_id.is_some()
        {
            panic!("Something inside object was not cleared properly");
        }
    }
}
