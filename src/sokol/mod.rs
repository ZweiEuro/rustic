mod input;
mod frame;

use std::ffi;

use sokol::{app::{self as sapp}, gfx as sg, glue as sglue};


use frame::SokolUserData;
use specs::World;

extern "C" fn cleanup(user_data: *mut ffi::c_void) {
    sg::shutdown();

    let _ = unsafe { Box::from_raw(user_data as *mut SokolUserData) };
}

extern "C" fn init(_user_data: *mut ffi::c_void) {

    sg::setup(&sg::Desc {
        environment: sglue::environment(),
        logger: sg::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        ..Default::default()
    });


    let backend = sg::query_backend();
    match &backend {
        sg::Backend::Glcore | sg::Backend::Gles3 => {
            println!("Using GL Backend!");
            println!("Specifically the {:?} backend!", backend);
        },

        sg::Backend::D3d11 => {
            println!("Using D3d11 Backend!");
        },

        sg::Backend::MetalIos | sg::Backend::MetalMacos | sg::Backend::MetalSimulator => {
            println!("Using Metal Backend!");
            println!("Specifically the {:?} backend!", backend);
        },

        sg::Backend::Wgpu => {
            println!("Using Wgpu Backend!");
        },

        sg::Backend::Dummy => {
            println!("Using Dummy Backend!");
        },
    }
}

pub fn sokol_main(world: World) {
    let state = Box::new(SokolUserData { world: world });

    let user_data = Box::into_raw(state) as *mut ffi::c_void;

    sapp::run(&sapp::Desc {
        init_userdata_cb: Some(init),
        frame_userdata_cb: Some(frame::frame),
        cleanup_userdata_cb: Some(cleanup),
        event_cb: Some(input::event_cb),
        user_data,
        window_title: c"clear.rs".as_ptr(),
        width: 800,
        height: 600,
        sample_count: 4,
        logger: sapp::Logger { func: Some(sokol::log::slog_func), ..Default::default() },
        icon: sapp::IconDesc { sokol_default: true, ..Default::default() },
enable_clipboard: true,
        ..Default::default()
    });
}
