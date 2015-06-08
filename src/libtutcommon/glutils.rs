#![doc = "Common stuff for OpenGL."]

use std;
use std::path::Path;
use std::ffi::{CString, CStr};
use std::fs::File;
use std::io::Read;

use gl;
use gl::types::{GLchar, GLuint};

    fn read_source_from_file<P: AsRef<Path>>(path : P) -> CString {
        let mut res = String::new();
        File::open(path).unwrap().read_to_string(&mut res).unwrap();
        return CString::new(res).unwrap();
    }

    #[doc = "Read shaders from file and build program."]
    pub fn load_program(vertex_file_path : &str, fragment_file_path : &str) -> GLuint {
        unsafe {
            // Create the shaders.
            let vertex_shader_id = gl::CreateShader(gl::VERTEX_SHADER);
            let fragment_shader_id = gl::CreateShader(gl::FRAGMENT_SHADER);
        
            // Read the Vertex Shader code from the file.
            let vertex_shader_code = read_source_from_file(vertex_file_path);
        
            // Read the Fragment Shader code from the file
            let fragment_shader_code = read_source_from_file(fragment_file_path);

            // Compile Vertex Shader
            println!("Compiling shader: {}", vertex_file_path);
            compile_and_check_shader(vertex_shader_id, &vertex_shader_code);

            // Compile Fragment Shader
            println!("Compiling shader: {}", fragment_file_path);
            compile_and_check_shader(fragment_shader_id, &fragment_shader_code);

            // Link the program
            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_shader_id);
            gl::AttachShader(program_id, fragment_shader_id);
            gl::LinkProgram(program_id);

            let mut result = 0;
            let mut info_log_length = 0;

            // Check the program
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut result);
            gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut info_log_length);
            let mut buf = Vec::<GLchar>::with_capacity((info_log_length + 1) as usize);
            buf.set_len((info_log_length + 1) as usize);
            gl::GetProgramInfoLog(program_id, info_log_length + 1, std::ptr::null_mut(), buf[..].as_mut_ptr());
            println!("Program link log: {}", String::from_utf8_lossy(std::mem::transmute::<&[i8],&[u8]>(&buf[..])));

            gl::DeleteShader(vertex_shader_id);
            gl::DeleteShader(fragment_shader_id);

            program_id
        }
    }

    fn compile_and_check_shader(shader_id : GLuint, shader_source : &CStr) {
        let mut result = 0;
        let mut info_log_length = 0;
        unsafe {
            let source : &[i8] = std::mem::transmute::<&[u8], &[i8]>(shader_source.to_bytes_with_nul());
            // Compile Shader
            gl::ShaderSource(shader_id, 1, std::mem::transmute::<&&[i8], *const *const i8>(&source), std::ptr::null());
            gl::CompileShader(shader_id);

            // Check Shader
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut result);
            gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut info_log_length);
            let mut buf = Vec::<GLchar>::with_capacity((info_log_length + 1) as usize);
            buf.set_len((info_log_length + 1) as usize);
            gl::GetShaderInfoLog(shader_id, info_log_length + 1, std::ptr::null_mut(), buf[..].as_mut_ptr());
            println!("Shader compile log: {}", String::from_utf8_lossy(std::mem::transmute::<&[i8],&[u8]>(&buf[..])));
        }
    }


