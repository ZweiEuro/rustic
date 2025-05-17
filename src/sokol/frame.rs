use specs::prelude::World;
use std::ffi;

use sokol::{app::{self as sapp, Event}, gfx as sg, glue as sglue, log};

pub struct SokolUserData {
    pub world: World,
}


pub extern "C" fn frame(user_data: *mut ffi::c_void) {
    let state = unsafe { &mut *(user_data as *mut SokolUserData) };

    let mut action = sg::PassAction::new();

    action.colors[0] = sg::ColorAttachmentAction {
        load_action: sg::LoadAction::Clear,
        clear_value: sg::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 },
        ..Default::default()
    };


    sg::begin_pass(&sg::Pass {
        action: action, 
        swapchain: sglue::swapchain(),
        ..Default::default()
    });
    sg::end_pass();
    sg::commit();
}



