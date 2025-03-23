use std::{
    sync::{LazyLock, Mutex},
    thread,
    time::Duration,
};

use object::IWorldObject;
use sdl3::video::Window;

pub mod cube;
pub mod object;

static OBJECT_MANAGER: LazyLock<Mutex<ObjectManager>> =
    LazyLock::new(|| Mutex::new(ObjectManager::new()));

const PHYSICS_FPS: i32 = 120;

struct ObjectManager {
    objects: Vec<Box<dyn IWorldObject>>,

    physics_thread_handle: thread::JoinHandle<()>,
}

impl ObjectManager {
    fn new() -> ObjectManager {
        // create a thread that automatically updates the inside objects

        let handle = thread::spawn(|| {
            let mut last_update = std::time::Instant::now();

            loop {
                {
                    let mut obj_m = OBJECT_MANAGER.lock().unwrap();
                    let timedelta = last_update.elapsed();

                    obj_m.objects.iter_mut().for_each(|object| {
                        object.physics_update(timedelta.as_secs_f32());
                    });

                    last_update = std::time::Instant::now();
                }

                thread::sleep(Duration::from_millis(
                    ((1.0 / PHYSICS_FPS as f32) * 1000.0) as u64,
                ));
            }
        });

        ObjectManager {
            objects: Vec::new(),
            physics_thread_handle: handle,
        }
    }
}

pub fn add_object<T: IWorldObject + 'static>(object: T) {
    OBJECT_MANAGER
        .lock()
        .unwrap()
        .objects
        .push(Box::new(object));
}

pub fn draw_all(canvas: &mut sdl3::render::Canvas<Window>) {
    OBJECT_MANAGER
        .lock()
        .unwrap()
        .objects
        .iter()
        .for_each(|f| f.draw(canvas));
}
