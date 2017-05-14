use std;

use gl;
use gl::types::{GLint, GLuint};

use sdl2;

use tutcommon::glutils;
use tutcommon::matrix::Vector3f;
use tutcommon::controls::Controls;
use tutcommon::objloader;
use tutcommon::objloader::Vector2f;

#[doc = "Moved out drawing GL stuff to avoid mess with the other code."]
pub struct GLScene {
    vertex_array_id: GLuint, //VAO id.
    vertex_buffer_id: GLuint, //VBO id.
    uv_buffer_id: GLuint, // UV id.
    program_id: GLuint, //Shader program id.
    texture_id: GLuint, // Texture id.
    matrix_uniform_id: GLint, // MVP uniform locaion.
    texture_uniform_id: GLint, // myTextureSampler uniform location.
}

impl GLScene {
    #[doc = "Create scene and init it."]
    pub fn new(vs: sdl2::VideoSubsystem) -> GLScene {

        let mut vertex_array_id = 0;

        unsafe {
            // create Vertex Array Object and set it as the current one:
            gl::GenVertexArrays(1, &mut vertex_array_id);
            gl::BindVertexArray(vertex_array_id);
        }

        // Create and compile our GLSL program from the shaders
        let program_id = glutils::load_program("data/tut07/TransformVertexShader.vertexshader",
                                               "data/tut07/TextureFragmentShader.fragmentshader");

        let mut vertices = Vec::new();
        let mut uvs = Vec::new();
        let mut normals = Vec::new();

        objloader::obj_load("data/tut07/cube.obj"
                            , &mut vertices
                            , &mut uvs
                            , &mut normals).expect("Load obj");
        println!("UV len: {}", uvs.len());
        println!("UV elem size: {}", std::mem::size_of::<Vector2f>());

        println!("Vertex len: {}", vertices.len());
        println!("Vertex elem size: {}", std::mem::size_of::<Vector3f>());

        let matrix_uniform_id = unsafe {
            // Get a handle for our "MVP" uniform
            gl::GetUniformLocation(program_id, "MVP\x00".as_ptr() as *const i8)
        };

        let texture_uniform_id = unsafe {
            gl::GetUniformLocation(program_id, "myTextureSampler\x00".as_ptr() as *const i8)
        };

        let mut vertex_buffer_id = 0;

        unsafe {
            // Generate 1 buffer.
            gl::GenBuffers(1, &mut vertex_buffer_id);

            // Set it current.
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_id);

            // Send vertices to buffer.
            gl::BufferData(gl::ARRAY_BUFFER,
                           (std::mem::size_of::<Vector3f>() * vertices.len()) as isize,
                           std::mem::transmute(vertices.as_ptr()),
                           gl::STATIC_DRAW);
        }

        let mut uv_buffer_id = 0;

        unsafe {
            gl::GenBuffers(1, &mut uv_buffer_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, uv_buffer_id);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (std::mem::size_of::<Vector2f>() * uvs.len()) as isize,
                           std::mem::transmute(uvs.as_ptr()),
                           gl::STATIC_DRAW);
        }

        let texture_id = glutils::load_dds_texture(&vs, "data/tut07/uvmap.DDS").unwrap();

        GLScene {
            vertex_array_id: vertex_array_id,
            vertex_buffer_id: vertex_buffer_id,
            uv_buffer_id: uv_buffer_id,
            texture_id: texture_id,
            program_id: program_id,
            matrix_uniform_id: matrix_uniform_id,
            texture_uniform_id: texture_uniform_id,
        }
    }

    #[doc = "Update data each frame."]
    pub fn update(&mut self) {}

    #[doc = "Render scene each frame."]
    pub fn draw(&self, controls: &Controls) {

        // Model matrix : an identity matrix (model will be at the origin)
        let model = std::default::Default::default();

        let mvp = controls.projection.mul(&controls.view).mul(&model);


        unsafe {
            // Use our shader
            gl::UseProgram(self.program_id);

            // Send our transformation to the currently bound shader,
            // in the "MVP" uniform.
            gl::UniformMatrix4fv(self.matrix_uniform_id, 1, gl::FALSE, mvp.as_ptr());

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
            // Set our "myTextureSampler" sampler to user Texture Unit 0
            gl::Uniform1i(self.texture_uniform_id, 0);

            //1st attribute buffer : vertices
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_id);
            gl::VertexAttribPointer(
                // attribute 0. No particular reason for 0, but must match the layout in the
                // shader.
                0,
                3, // size
                gl::FLOAT, // type
                gl::FALSE, // normalized?
                0, // stride
                std::ptr::null() // array buffer offset
            );

            // 2nd attribute buffer : colors
            gl::EnableVertexAttribArray(1);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.uv_buffer_id);
            gl::VertexAttribPointer(
                1, // attribute 1.
                2, // size
                gl::FLOAT, // type
                gl::FALSE, // normalized?
                0, // stride
                std::ptr::null() // array buffer offset
            );

            // Draw the triangle!
            // 12*3 indices starting at 0 -> 12 triangles -> 6 squares
            gl::DrawArrays(gl::TRIANGLES, 0, 12 * 3);

            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
        }
    }
}

#[doc = "Always clean up after yourself."]
impl Drop for GLScene {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);

            gl::DeleteBuffers(1, &self.vertex_buffer_id);
            gl::DeleteBuffers(1, &self.uv_buffer_id);

            gl::DeleteTextures(1, &self.texture_id);

            gl::DeleteVertexArrays(1, &self.vertex_array_id);
        }
    }
}
