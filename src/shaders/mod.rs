use std::fs; use std::path::Path;
use notify::{recommended_watcher, Event, RecursiveMode, Result, Watcher};

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
        ret.start_watching();

        return ret;
    }

    fn start_watching(&self) {

        fn event_fn(res: Result<notify::Event>) {
            match res {
                Ok(event) => println!("event: {:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
        let mut watcher = notify::recommended_watcher(event_fn);

        if let Err(e) = watcher {
            println!("Error creating file watcher {}", e);
            panic!("could not create watcher")
        }


        let mut watcher = watcher.unwrap();
        let res = watcher.watch(std::path::Path::new("./shaders/"), RecursiveMode::Recursive);


        if let Err(e) = res {
            println!("Error watching path {}", e);
            panic!("could not create watcher")
        }
    println!("watching path");
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

    fn drop(&mut self) {
        println!("{}", self.name);
    }

}



