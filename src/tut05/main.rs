#![deny(deprecated)]
#![deny(missing_docs)]
#![deny(non_snake_case)]
#![deny(non_upper_case_globals)]

#![crate_name = "tut05"]

#![doc = "http://www.opengl-tutorial.org/beginners-tutorials/tutorial-5-a-textured-cube/"]

// Include SDL2 library.
extern crate sdl2;
// Include OpenGL library.
extern crate gl;

extern crate libc;

extern crate rand;

use gl::types::{GLenum, GLuint, GLsizei, GLchar};
use libc::types::common::c95::c_void;
use std::ffi::CStr;

extern crate tutcommon;

#[doc = "Module for GL drawing stuff."]
pub mod glscene;

extern "system" fn on_debug_message(_source: GLenum, _gltype: GLenum, _id: GLuint, _severity: GLenum, _length: GLsizei, message: *const GLchar, _: *mut c_void) {
    let msg = unsafe {
        String::from_utf8_lossy(CStr::from_ptr(message).to_bytes())
    };

    println!("[OpenGL] {}", msg);
}

fn main() {
    // Initialize SDL2:
    let mut sdl_context = sdl2::InitBuilder::new().video().unwrap();

    // Init OpenGL parameters:
    sdl2::video::gl_attr::set_multisample_buffers(1);
    sdl2::video::gl_attr::set_multisample_samples(4); // 4x antialiasing
    sdl2::video::gl_attr::set_context_version(3, 3); // OpenGL 3.3
    sdl2::video::gl_attr::set_context_flags().debug().set();
    sdl2::video::gl_attr::set_context_profile(sdl2::video::GLProfile::Core); // Don't use old OpenGL

    let window = sdl_context.window("Tutorial 05", 1024, 768).position_centered().opengl().build().unwrap();

    let _gl_context = window.gl_create_context().unwrap();

    gl::load_with(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    });

    if sdl2::video::gl_extension_supported("ARB_debug_support") {
        unsafe {
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageControl(gl::DONT_CARE, gl::DONT_CARE, gl::DONT_CARE, 0, std::ptr::null(), gl::TRUE);
            gl::DebugMessageCallback(on_debug_message, std::ptr::null());
        }
    }

    unsafe {
        gl::ClearColor(0.0, 0.0, 0.4, 0.0);
        // Enable depth test
        gl::Enable(gl::DEPTH_TEST);
        // Accept fragment if it closer to the camera than the former one
        gl::DepthFunc(gl::LESS);
    }

    // init scene.
    let mut scene = glscene::GLScene::new();

    let mut event_pump = sdl_context.event_pump();
    'evloop : loop {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
        scene.update();

        scene.draw();

        // Swap buffers.
        window.gl_swap_window();

        for event in event_pump.poll_iter() {
            // check if ESC key pressed or windows closed.
            match event {
                sdl2::event::Event::Quit { .. } => {
                    return;
                }
                sdl2::event::Event::KeyDown { 
                            timestamp: _,
                            window_id: _,
                            keycode: _,
                            scancode,
                            keymod: _,
                            repeat: _, 
                        } => {
                    if scancode == Some(sdl2::keyboard::Scancode::Escape) {
                        return;
                    }
                }
                _ => ()
            }
        }
    }
}

