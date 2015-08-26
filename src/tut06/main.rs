#![deny(deprecated)]
#![deny(missing_docs)]
#![deny(non_snake_case)]
#![deny(non_upper_case_globals)]

#![crate_name = "tut06"]

#![doc = "http://www.opengl-tutorial.org/beginners-tutorials/tutorial-6-keyboard-and-mouse/"]

// Include SDL2 library.
extern crate sdl2;
// Include OpenGL library.
extern crate gl;

extern crate libc;

extern crate rand;

extern crate tutcommon;

use tutcommon::sdl;

#[doc = "Module for GL drawing stuff."]
pub mod glscene;

fn main() {
    let mut sdl_context = sdl::SdlContext::init("Tutorial 06");

    unsafe {
        gl::ClearColor(0.0, 0.0, 0.4, 0.0);
        // Enable depth test
        gl::Enable(gl::DEPTH_TEST);
        // Accept fragment if it closer to the camera than the former one
        gl::DepthFunc(gl::LESS);
    }

    // init scene.
    let mut scene = glscene::GLScene::new(sdl_context.vs);

    'evloop : loop {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
        scene.update();

        scene.draw();

        // Swap buffers.
        sdl_context.window.gl_swap_window();

        for event in sdl_context.event_pump.poll_iter() {
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

