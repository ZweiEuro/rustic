use sokol::app::{Event, EventType, Keycode};
use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};


static HELD_KEYS: LazyLock<Mutex<HashSet<Keycode>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

pub fn key_is_pressed(key: Keycode) -> bool{
    return HELD_KEYS.lock().unwrap().contains(&key);
}


pub extern "C" fn event_cb(event: *const Event){

    let event = unsafe { &mut *(event as *mut Event) };

    match event._type {
        EventType::Invalid =>{
            println!("Invalid input event");
        }

        EventType::KeyUp =>{
            HELD_KEYS.lock().unwrap().remove(&event.key_code); 
            println!("Set {:?}", HELD_KEYS.lock().unwrap().clone());
        }

        EventType::KeyDown => {
            HELD_KEYS.lock().unwrap().insert(event.key_code); 
        }

        _ => {
        }
    }

}
