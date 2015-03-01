#![deny(deprecated)]
#![deny(missing_docs)]
#![deny(non_snake_case)]
#![deny(non_upper_case_globals)]

#![feature(path)]

#![crate_name = "tut03"]

#![doc = "http://www.opengl-tutorial.org/beginners-tutorials/tutorial-3-matrices/"]

// Include SDL2 library.
extern crate sdl2;
// Include OpenGL library.
extern crate gl;

extern crate tutcommon;

#[doc = "Module for GL drawing stuff."]
pub mod glscene;

fn main() {
    // Initialize SDL2:
    let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();

    // Init OpenGL parameters:
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLMultiSampleBuffers, 1);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLMultiSampleSamples, 4); // 4x antialiasing
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMajorVersion, 3); // OpenGL 3.3
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMinorVersion, 3);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextProfileMask
        , sdl2::video::GLProfile::GLCoreProfile as i32); // Don't use old OpenGL

    let window = sdl2::video::Window::new("Tutorial 03", sdl2::video::WindowPos::PosCentered
        , sdl2::video::WindowPos::PosCentered, 1024, 768, sdl2::video::OPENGL).unwrap();

    let gl_context = window.gl_create_context().unwrap();

    gl::load_with(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    });

    unsafe {
        gl::ClearColor(0.0, 0.0, 0.4, 0.0);
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

