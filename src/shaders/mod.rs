use std::fs;
use miniquad::ShaderSource; 

pub struct Shader {
    name: String,

    vertex_string_contents: String,
    fragment_string_contents: String


}


impl Shader {

    pub fn new(basename: String) -> Self{
        let mut ret = Shader { name: basename, vertex_string_contents: String::from(""), 
            fragment_string_contents:String::from("")};

        ret.load_from_disk();

        return ret;
    }

    fn load_from_disk(&mut self) {
        let frag_path = format!("./shaders/{}.frag.glsl", self.name);
        let vert_path = format!("./shaders/{}.vert.glsl", self.name);

        println!("{} {}",frag_path, vert_path);

        self.fragment_string_contents = fs::read_to_string(frag_path)
            .expect("Should have been able to read the file");

        self.vertex_string_contents = fs::read_to_string(vert_path)
            .expect("Should have been able to read the file");

        println!("frag shader content:\n{}", self.fragment_string_contents); 
    }

pub fn get_shadersource(&self) -> ShaderSource{
        return ShaderSource::Glsl{ vertex: &self.vertex_string_contents, fragment: &self.fragment_string_contents};
    }


}



