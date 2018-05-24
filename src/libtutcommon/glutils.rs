#![doc = "Common stuff for OpenGL."]

use std::{
    self, ffi::{CStr, CString, OsStr}, fs::File, io::{Read, Result}, path::Path,
};

use byteorder::{ByteOrder, LittleEndian};

use gl::{
    self, types::{GLchar, GLuint, GLvoid},
};

use sdl2;

const FOURCC_DXT1: u32 = 0x3154_5844; // Equivalent to "DXT1" in ASCII
const FOURCC_DXT3: u32 = 0x3354_5844; // Equivalent to "DXT3" in ASCII
const FOURCC_DXT5: u32 = 0x3554_5844; // Equivalent to "DXT5" in ASCII

fn read_source_from_file<P: AsRef<Path>>(path: P) -> CString {
    let mut res = String::new();
    File::open(path)
        .expect("Open file")
        .read_to_string(&mut res)
        .expect("Read file");
    CString::new(res).unwrap()
}

#[doc = "Read shaders from file and build program."]
pub fn load_program(vertex_file_path: &str, fragment_file_path: &str) -> GLuint {
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
        gl::GetProgramInfoLog(
            program_id,
            info_log_length + 1,
            std::ptr::null_mut(),
            buf[..].as_mut_ptr(),
        );
        println!(
            "Program link log: {}",
            String::from_utf8_lossy(&*(&buf[..] as *const _ as *const [u8]))
        );

        gl::DeleteShader(vertex_shader_id);
        gl::DeleteShader(fragment_shader_id);

        program_id
    }
}

fn compile_and_check_shader(shader_id: GLuint, shader_source: &CStr) {
    let mut result = 0;
    let mut info_log_length = 0;
    unsafe {
        let source: &[i8] = &*(shader_source.to_bytes_with_nul() as *const _ as *const [i8]);
        // Compile Shader
        gl::ShaderSource(
            shader_id,
            1,
            &source as *const &[i8] as *const *const i8,
            std::ptr::null(),
        );
        gl::CompileShader(shader_id);

        // Check Shader
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut result);
        gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut info_log_length);
        let mut buf = Vec::<GLchar>::with_capacity((info_log_length + 1) as usize);
        buf.set_len((info_log_length + 1) as usize);
        gl::GetShaderInfoLog(
            shader_id,
            info_log_length + 1,
            std::ptr::null_mut(),
            buf[..].as_mut_ptr(),
        );
        println!(
            "Shader compile log: {}",
            String::from_utf8_lossy(&*(&buf[..] as *const _ as *const [u8]))
        );
    }
}

fn load_bmp<S: AsRef<OsStr> + ?Sized>(s: &S) -> sdl2::surface::Surface {
    sdl2::surface::Surface::load_bmp(Path::new(s)).unwrap()
}

#[doc = "Load BMP texture from file path"]
pub fn load_bmp_texture(file: &str) -> GLuint {
    let surface = load_bmp(file);

    let (width, height) = surface.size();

    surface.pixel_format();

    let mut texture_id = 0;
    surface.with_lock(|data| unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            width as i32,
            height as i32,
            0,
            gl::BGR,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const GLvoid,
        );

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
    });

    texture_id
}

#[doc = "Load DDS texture from file path"]
pub fn load_dds_texture(vs: &sdl2::VideoSubsystem, file: &str) -> Result<GLuint> {
    if !vs.gl_extension_supported("GL_EXT_texture_compression_s3tc") {
        panic!("S3TC not supported.");
    }

    /* from https://www.opengl.org/registry/specs/EXT/texture_compression_s3tc.txt */
    //const COMPRESSED_RGB_S3TC_DXT1_EXT : GLuint = 0x83F0;
    const COMPRESSED_RGBA_S3TC_DXT1_EXT: GLuint = 0x83F1;
    const COMPRESSED_RGBA_S3TC_DXT3_EXT: GLuint = 0x83F2;
    const COMPRESSED_RGBA_S3TC_DXT5_EXT: GLuint = 0x83F3;

    let mut f: File = File::open(file)?;
    let mut sign4: [u8; 4] = [0; 4];

    if f.read(&mut sign4)? != 4 {
        panic!("Cann't read 4 bytes");
    }
    if &sign4[..] != b"DDS " {
        panic!("Wrong signature");
    }

    let mut header: [u8; 124] = [0; 124];
    if f.read(&mut header)? != 124 {
        panic!("Cann't read 124 bytes");
    }

    let mut height: i32 = LittleEndian::read_u32(&header[8..]) as i32;
    let mut width: i32 = LittleEndian::read_u32(&header[12..]) as i32;
    let linear_size: usize = LittleEndian::read_u32(&header[16..]) as usize;
    let mip_map_count: i32 = LittleEndian::read_u32(&header[24..]) as i32;
    let four_cc: u32 = LittleEndian::read_u32(&header[80..]);

    println!(
        "h {} w {} ls {} mmc {} fcc {}",
        height, width, linear_size, mip_map_count, four_cc
    );

    let bufsize: usize = if mip_map_count > 1 {
        linear_size * 2
    } else {
        linear_size
    };
    let mut buffer: Vec<u8> = Vec::with_capacity(bufsize);

    f.read_to_end(&mut buffer)?;

    //let components = if four_cc == FOURCC_DXT1 { 3 } else { 4 };
    let format = match four_cc {
        FOURCC_DXT1 => COMPRESSED_RGBA_S3TC_DXT1_EXT,
        FOURCC_DXT3 => COMPRESSED_RGBA_S3TC_DXT3_EXT,
        FOURCC_DXT5 => COMPRESSED_RGBA_S3TC_DXT5_EXT,
        _ => unimplemented!(),
    };

    let mut texture_id = 0;
    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);

        let block_size = if format == COMPRESSED_RGBA_S3TC_DXT1_EXT {
            8
        } else {
            16
        };
        let mut offset = 0;
        let mut level = 0i32;
        while level < mip_map_count && (width > 0 || height > 0) {
            let size = (((width + 3) / 4) * ((height + 3) / 4) * block_size) as usize;

            gl::CompressedTexImage2D(
                gl::TEXTURE_2D,
                level,
                format,
                width,
                height,
                0,
                size as i32,
                (&buffer[offset..]).as_ptr() as *const GLvoid,
            );

            offset += size;
            width /= 2;
            height /= 2;
            level += 1;
        }
    }

    Ok(texture_id)
}
