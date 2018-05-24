#![doc = "Common stuff for SDL2."]

use sdl2;

use std;

use gl;

use gl::types::{GLchar, GLenum, GLsizei, GLuint, GLvoid};
use std::ffi::CStr;

#[doc = "Context of SDL2."]
pub struct SdlContext {
    #[doc = "SDL2 itself"]
    pub sdl: sdl2::Sdl,

    #[doc = "SDL2 video subsystem"]
    pub vs: sdl2::VideoSubsystem,

    #[doc = "SDL2 timer subsystem"]
    pub ts: sdl2::TimerSubsystem,

    #[doc = "SDL2 window"]
    pub window: sdl2::video::Window,

    #[doc = "SDL2 event system"]
    pub event_pump: sdl2::EventPump,

    _gl_context: sdl2::video::GLContext,
}

extern "system" fn on_debug_message(
    _source: GLenum,
    _gltype: GLenum,
    _id: GLuint,
    _severity: GLenum,
    _length: GLsizei,
    message: *const GLchar,
    _: *mut GLvoid,
) {
    let msg = unsafe { String::from_utf8_lossy(CStr::from_ptr(message).to_bytes()) };

    println!("[OpenGL] {}", msg);
}

impl SdlContext {
    #[doc = "Initialize common sdl2 stuff"]
    pub fn init(window_name: &str) -> SdlContext {
        // Initialize SDL2:
        let sdl_context = sdl2::init().unwrap();
        let sdl_vs_context = sdl_context.video().unwrap();
        let sdl_ts_context = sdl_context.timer().unwrap();

        // Init OpenGL parameters:
        sdl_vs_context.gl_attr().set_multisample_buffers(1);
        sdl_vs_context.gl_attr().set_multisample_samples(4); // 4x antialiasing
        sdl_vs_context.gl_attr().set_context_version(3, 3); // OpenGL 3.3
        sdl_vs_context.gl_attr().set_context_flags().debug().set();
        // Don't use old OpenGL
        sdl_vs_context
            .gl_attr()
            .set_context_profile(sdl2::video::GLProfile::Core);

        let window = sdl_vs_context
            .window(window_name, 1024, 768)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let _gl_context = window.gl_create_context().unwrap();

        gl::load_with(|s| sdl_vs_context.gl_get_proc_address(s) as *const GLvoid );

        if sdl_vs_context.gl_extension_supported("ARB_debug_support") {
            unsafe {
                gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
                gl::DebugMessageControl(
                    gl::DONT_CARE,
                    gl::DONT_CARE,
                    gl::DONT_CARE,
                    0,
                    std::ptr::null(),
                    gl::TRUE,
                );
                gl::DebugMessageCallback(on_debug_message, std::ptr::null());
            }
        }

        let event_pump = sdl_context.event_pump().unwrap();

        SdlContext {
            sdl: sdl_context,
            vs: sdl_vs_context,
            ts: sdl_ts_context,
            window,
            _gl_context,
            event_pump,
        }
    }
}
