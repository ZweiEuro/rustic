use std::{fs::File, io::Read, panic};

/// represents a file loaded with stb_image_rust
pub struct Texture {
    pub _name: String,
    pub _file_contents: Vec<u8>,

    pub length: i32,
    pub width: i32,
    pub height: i32,
    pub comp: i32,

    pub img: *mut u8,
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
        return ret;
    }
}

// must be deallocated correctly when dropped
impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            stb_image_rust::c_runtime::free(self.img);
        }
    }
}
