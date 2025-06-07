use std::{fs::File, io::Read, panic};

use miniquad::{RenderingBackend, TextureId};

/*
       // Load file into memory

    // Do something with it
    ...

    // Free the allocated memory
        unsafe {
            stb_image_rust::c_runtime::free(img);
        }
*/
pub struct Texture {
    pub _name: String,
    pub _file_contents: Vec<u8>,

    pub length: i32,
    pub width: i32,
    pub height: i32,
    pub comp: i32,

    img: *mut u8,

    // opengl stuff
    texture_id: Option<TextureId>,
}

impl Texture {
    pub fn new(basename: String) -> Self {
        let mut ret = Self {
            _name: basename.to_owned(),
            _file_contents: vec![],

            length: 0,
            width: 0,
            height: 0,

            comp: 0,
            img: 0 as *mut u8,

            texture_id: None,
        };

        let path = format!("./sprites/{}", basename);
        println!("trying to load texture {}", path);
        let mut f = File::open(path.clone()).expect("Error loading texture");

        let mut contents = vec![];
        let err = f.read_to_end(&mut contents);

        if err.is_err() {
            panic!("could not load {}", basename);
        }

        unsafe {
            stb_image_rust::stbi_set_flip_vertically_on_load(1);
            ret.img = stb_image_rust::stbi_load_from_memory(
                contents.as_mut_ptr(),
                contents.len() as i32,
                &mut ret.width,
                &mut ret.height,
                &mut ret.comp,
                stb_image_rust::STBI_rgb_alpha,
            );
        }

        ret.length = contents.len() as i32;
        println!("w {} h {} l {}", ret.width, ret.height, ret.length);
        return ret;
    }

    pub fn get_texture_id(&mut self, ctx: &mut Box<dyn RenderingBackend>) -> TextureId {
        if self.texture_id.is_none() {
            self.texture_id = Some(ctx.new_texture_from_rgba8(
                self.width as u16,
                self.width as u16,
                unsafe {
                    std::slice::from_raw_parts(self.img, (self.width * self.height * 4) as usize)
                },
            ));
        }
        return self.texture_id.unwrap();
    }

    pub fn delete_texture(&mut self, ctx: &mut Box<dyn RenderingBackend>) {
        if self.texture_id.is_none() == false {
            ctx.delete_texture(self.texture_id.unwrap());
            self.texture_id = None;
        }
    }
}

// must be deallocated correctly when dropped
impl Drop for Texture {
    fn drop(&mut self) {
        if self.texture_id.is_none() == false {
            panic!("Texture dropped before it was cleared from the GPU!");
        }

        unsafe {
            stb_image_rust::c_runtime::free(self.img);
        }
    }
}
