use futures::{
    channel::mpsc::{channel, Receiver, TryRecvError}, executor::block_on, SinkExt, StreamExt
};
use notify::{event::{ModifyKind}, Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::{fs};
use miniquad::{ShaderSource};

/**
* Load a shader from  disk and automatically convert it to formats that miniquad can understand
*/
pub struct Shader {
    name: String,

    frag_path: String,
    vert_path: String,

    vertex_string_contents: String,
    fragment_string_contents: String,

    rx_file_watcher_detect_change: Receiver<bool>,

}

/**
* Note: This implementation is NOT ideal. 
* It starts a thread that watches for file changes, but that thread will run indefinitely 
* Only when the program stops is the thread actually "freed"
*
* I should probably look more into what the whatcher does and how it works but for now this is good
* enough
* How to do this!: You can use one notifier/sender tx and clone it to be used by notify internally!
* When we send our notification we can differenciate it via an enum and it should be fine!
*/
impl Shader {

    pub fn new(basename: String) -> Self{
        let frag_path = format!("./shaders/{}.frag.glsl", basename);
        let vert_path = format!("./shaders/{}.vert.glsl", basename);
 
        let (mut tx_change_detected, rx_change_detected) = channel(1);

        let mut ret = Shader { name: basename, vertex_string_contents: String::from(""), 
            fragment_string_contents:String::from(""),
            frag_path : frag_path.clone(),
            vert_path: vert_path.clone(),
            rx_file_watcher_detect_change: rx_change_detected
        };

        ret.load_from_disk();

        // gotten directly from the example code from notify crate! 
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

        // Making this into async would make sense but then returns a future that is useless to us,
        // again
        std::thread::spawn(move || {
            let (mut watcher, mut rx) = async_watcher().unwrap();

            // Add a path to be watched. All files and directories at that path and
            // below will be monitored for changes.
            watcher.watch(frag_path.as_ref(), RecursiveMode::NonRecursive).unwrap();
            watcher.watch(vert_path.as_ref(), RecursiveMode::NonRecursive).unwrap();

            //Note: "block_on" is like await but for non async functions
            while let Some(res) = block_on(rx.next()) {
                match res {
                    Ok(event) => {
                        match event.kind {
                            EventKind::Modify(modify_event) => {
                                // println!("modify event {:?}", modify_event);
                                match modify_event {
                                    ModifyKind::Metadata(..) => {
                                        // println!("Shader must be reloaded!"); 
                                        drop(block_on(tx_change_detected.send(true)));
                                    }
                                    _ => (),
                                }
                            }
                            EventKind::Remove(..) => {
                                // Vim fully removes files when writing; that means it destroys the
                                // filesystem node making notify unable to track it. we need to
                                // re-watch it when that happens. it also qualifies as a change but
                                // that should always trigger a normal modify as well.

                                for filepath in event.paths.iter(){
                                    // do NOT unwatch it, apparently notify does that internally
                                    // which will crash if we do it twice since its not watching it
                                    // anymore at this point
                                    watcher.watch(&filepath,RecursiveMode::NonRecursive).unwrap();
                                }
                            }
                            _ => (),
                        }
                    },
                    Err(e) => panic!("watch error: {:?}", e),
                }
            }
        });

        return ret;
    }

    /** Reload the code if it changed on the disk
    * return true if there actually was a code change, indicating that the shader needs to be
    * reloaded
    */
    pub fn reload_if_needed(&mut self) -> bool{
        match self.rx_file_watcher_detect_change.try_next() {
            Ok(None) => {}
            Ok(res) => {
                self.load_from_disk();
                return true;
            }
            Err(_) => { }
        }
        return false;
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



