use std::time::Instant;

use sdl3::{EventPump, event::Event, keyboard::Keycode, libc::printf};
use specs::prelude::*;

use crate::{
    SysState,
    components::{InputMovement, Physics},
};

#[derive(SystemData)]
pub struct InputComp<'a> {
    sysState: Write<'a, SysState>,
    physics: WriteStorage<'a, Physics>,
    input_handler: WriteStorage<'a, InputMovement>,
}

pub struct SysInput {
    pub event_pump: EventPump,
}

unsafe impl Sync for SysInput {}
unsafe impl Send for SysInput {}

impl<'a> System<'a> for SysInput {
    type SystemData = InputComp<'a>;

    fn run(&mut self, mut data: InputComp) {
        let mut sysState = data.sysState;

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    sysState.running = false;
                }
                _ => {}
            }

            for obj in (&mut data.physics, &mut data.input_handler).join() {
                if (obj.1.handler)(event.clone(), obj.0, obj.1) == true {
                    break;
                }
            }
        }
    }
}
