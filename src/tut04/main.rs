#![deny(deprecated)]
#![deny(missing_docs)]
#![deny(non_snake_case)]
#![deny(non_upper_case_globals)]

#![crate_name = "tut04"]

#![doc = "http://www.opengl-tutorial.org/beginners-tutorials/tutorial-4-a-colored-cube/"]

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

extern "system" fn on_debug_message(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, user_param: *mut c_void) {
    let msg = unsafe {
        String::from_utf8_lossy(CStr::from_ptr(message).to_bytes())
    };

    println!("[OpenGL] {}", msg);
}

fn main() {
    // Initialize SDL2:
    let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();

    // Init OpenGL parameters:
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLMultiSampleBuffers, 1);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLMultiSampleSamples, 4); // 4x antialiasing
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMajorVersion, 3); // OpenGL 3.3
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMinorVersion, 3);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextFlags, sdl2::video::GL_CONTEXT_DEBUG.bits());
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextProfileMask
        , sdl2::video::GLProfile::GLCoreProfile as i32); // Don't use old OpenGL

    let window = sdl2::video::Window::new(&sdl_context, "Tutorial 04", sdl2::video::WindowPos::PosCentered
        , sdl2::video::WindowPos::PosCentered, 1024, 768, sdl2::video::OPENGL).unwrap();

    let gl_context = window.gl_create_context().unwrap();

    gl::load_with(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    });

    unsafe {
        gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
        gl::DebugMessageControl(gl::DONT_CARE, gl::DONT_CARE, gl::DONT_CARE, 0, std::ptr::null(), gl::TRUE);
        gl::DebugMessageCallback(on_debug_message, std::ptr::null());
    }

    unsafe {
        gl::ClearColor(0.0, 0.0, 0.4, 0.0);
        // Enable depth test
        gl::Enable(gl::DEPTH_TEST);
        // Accept fragment if it closer to the camera than the former one
        gl::DepthFunc(gl::LESS);
    }

    // init scene.
    let scene = glscene::GLScene::new();

    let mut event_pump = sdl_context.event_pump();
    'evloop : loop {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
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
                    if scancode == sdl2::scancode::ScanCode::Escape {
                        return;
                    }
                }
                _ => ()
            }
        }
    }
}

