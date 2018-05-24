use std;

use gl::{
    self, types::{GLfloat, GLuint, GLvoid},
};

use tutcommon::glutils;

// An array of 3 vectors which represents 3 vertices.
static G_VERTEX_BUFFER_DATA: [GLfloat; 9] = [-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0];

#[doc = "Moved out drawing GL stuff to avoid mess with the other code."]
#[derive(Default)]
pub struct GLScene {
    vertex_array_id: GLuint,  //VAO id.
    vertex_buffer_id: GLuint, //VBO id.
    program_id: GLuint,       //Shader program id.
}

impl GLScene {
    #[doc = "Create scene and init it."]
    pub fn new() -> GLScene {
        let mut vertex_array_id = 0;

        unsafe {
            // create Vertex Array Object and set it as the current one:
            gl::GenVertexArrays(1, &mut vertex_array_id);
            gl::BindVertexArray(vertex_array_id);
        }

        // Create and compile our GLSL program from the shaders
        let program_id = glutils::load_program(
            "data/tut02/SimpleVertexShader.vertexshader",
            "data/tut02/SimpleFragmentShader.fragmentshader",
        );

        let mut vertex_buffer_id = 0;

        unsafe {
            // Generate 1 buffer.
            gl::GenBuffers(1, &mut vertex_buffer_id);

            // Set it current.
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_id);

            // Send vertices to buffer.
            gl::BufferData(
                gl::ARRAY_BUFFER,
                std::mem::size_of_val(&G_VERTEX_BUFFER_DATA) as isize,
                &G_VERTEX_BUFFER_DATA as *const _ as *const GLvoid,
                gl::STATIC_DRAW,
            );
        }

        GLScene {
            vertex_array_id,
            vertex_buffer_id,
            program_id,
        }
    }

    #[doc = "Render scene each frame."]
    pub fn draw(&self) {
        unsafe {
            //1st attribute buffer : vertices
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_id);
            gl::VertexAttribPointer(
                // attribute 0. No particular reason for 0, but must match the
                // layout in the shader.
                0,
                3,                // size
                gl::FLOAT,        // type
                gl::FALSE,        // normalized?
                0,                // stride
                std::ptr::null(), // array buffer offset
            );

            // Use our shader
            gl::UseProgram(self.program_id);

            // Draw the triangle!
            // Starting from vertex 0; 3 vertices total -> 1 triangle.
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            gl::DisableVertexAttribArray(0);
        }
    }
}

#[doc = "Always clean up after yourself."]
impl Drop for GLScene {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);

            gl::DeleteBuffers(1, &self.vertex_buffer_id);

            gl::DeleteVertexArrays(1, &self.vertex_array_id);
        }
    }
}
