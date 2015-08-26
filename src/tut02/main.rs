#![deny(deprecated)]
#![deny(missing_docs)]
#![deny(non_snake_case)]
#![deny(non_upper_case_globals)]

#![crate_name = "tut02"]

#![doc = "http://www.opengl-tutorial.org/beginners-tutorials/tutorial-2-the-first-triangle/"]

// Include SDL2 library.
extern crate sdl2;
// Include OpenGL library.
extern crate gl;

extern crate tutcommon;

use tutcommon::sdl;

#[doc = "Module for GL drawing stuff."]
pub mod glscene;

fn main() {
    let mut sdl_context = sdl::SdlContext::init("Tutorial 02");

    unsafe {
        gl::ClearColor(0.0, 0.0, 0.4, 0.0);
    }

    // init scene.
    let scene = glscene::GLScene::new();

    'evloop : loop {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
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

