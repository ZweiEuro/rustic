use futures::{
    channel::mpsc::{channel, Receiver}, executor::block_on, SinkExt, StreamExt
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{fs, path::Path};
use miniquad::{ShaderSource};

/**
* Load a shader from  disk and automatically convert it to formats that miniquad can understand
*/
pub struct Shader {
    name: String,

    frag_path: String,
    vert_path: String,

    vertex_string_contents: String,
    fragment_string_contents: String
}


impl Shader {

    pub fn new(basename: String) -> Self{
        let frag_path = format!("./shaders/{}.frag.glsl", basename);
        let vert_path = format!("./shaders/{}.vert.glsl", basename);
 

        let mut ret = Shader { name: basename, vertex_string_contents: String::from(""), 
            fragment_string_contents:String::from(""),
            frag_path : frag_path.clone(),
            vert_path: vert_path.clone(),
        };

        ret.load_from_disk();

        fn async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
            let (mut tx, rx) = channel(1);

            // Automatically select the best implementation for your platform.
            // You can also access each implementation directly e.g. INotifyWatcher.
            let watcher = RecommendedWatcher::new(
                move |res| {
                    futures::executor::block_on(async {
                        tx.send(res).await.unwrap();
                    })
                },
                Config::default(),
            )?;

            Ok((watcher, rx))
        }

        std::thread::spawn(move|| {
            let (mut watcher, mut rx) = async_watcher().unwrap();



            // Add a path to be watched. All files and directories at that path and
            // below will be monitored for changes.
            watcher.watch(frag_path.as_ref(), RecursiveMode::NonRecursive).unwrap();
            watcher.watch(vert_path.as_ref(), RecursiveMode::NonRecursive).unwrap();
       
            while let Some(res) = block_on(rx.next()) {
                match res {
                    Ok(event) => println!("changed: {:?}", event),
                    Err(e) => println!("watch error: {:?}", e),
                }
            }
        });

        return ret;
    }

    /** Load the shader from disk and into strings to hold the text information inside of em
    */
    fn load_from_disk(&mut self) {
        let frag_path = self.frag_path.clone();
        let vert_path = self.vert_path.clone();


        self.fragment_string_contents = fs::read_to_string(frag_path)
            .expect("Should have been able to read the file");

        self.vertex_string_contents = fs::read_to_string(vert_path)
            .expect("Should have been able to read the file");
    }

    pub fn get_shadersource(&self) -> ShaderSource{
        return ShaderSource::Glsl{ vertex: &self.vertex_string_contents, fragment: &self.fragment_string_contents};
    }


}



