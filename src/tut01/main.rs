#![deny(deprecated)]
#![deny(missing_docs)]
#![deny(non_snake_case)]
#![deny(non_upper_case_globals)]

#![crate_name = "tut01"]

#![doc = "http://www.opengl-tutorial.org/beginners-tutorials/tutorial-1-opening-a-window/"]

// Include SDL2 library.
extern crate sdl2;
// Include OpenGL library.
extern crate gl;

fn main() {
    // Initialize SDL2:
    let mut sdl_context = sdl2::InitBuilder::new().video().unwrap();

    // Init OpenGL parameters:
    sdl2::video::gl_attr::set_multisample_buffers(1);
    sdl2::video::gl_attr::set_multisample_samples(4); // 4x antialiasing
    sdl2::video::gl_attr::set_context_version(3, 3); // OpenGL 3.3
    sdl2::video::gl_attr::set_context_profile(sdl2::video::GLProfile::Core); // Don't use old OpenGL

    let window = sdl_context.window("Tutorial 01", 1024, 768).position_centered().opengl().build().unwrap();
    
    let _gl_context = window.gl_create_context().unwrap();

    gl::load_with(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    });

    let mut event_pump = sdl_context.event_pump();
    'evloop : loop {
        // Draw nothing. Next in tutorial 2.

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

