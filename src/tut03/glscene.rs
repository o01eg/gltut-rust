use std;

use gl;
use gl::types::{GLfloat, GLint, GLuint};

use tutcommon;
use tutcommon::matrix::{Matrix4f, Vector3f};

// An array of 3 vectors which represents 3 vertices.
static G_VERTEX_BUFFER_DATA: [GLfloat; 9] = [-1.0, -1.0, 0.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0];

#[doc = "Moved out drawing GL stuff to avoid mess with the other code."]
pub struct GLScene {
    vertex_array_id: GLuint, //VAO id.
    vertex_buffer_id: GLuint, //VBO id.
    program_id: GLuint, //Shader program id.
    matrix_id: GLint, // MVP uniform locaion.
    mvp: Matrix4f, // Matrix
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

        let mut vertex_buffer_id = 0;

        // Create and compile our GLSL program from the shaders
        let program_id = tutcommon::glutils::load_program("data/tut03/SimpleTransform.vertexshader",
                                                          "data/tut03/SingleColor.fragmentshader");

        let matrix_id = unsafe {
            // Get a handle for our "MVP" uniform
            gl::GetUniformLocation(program_id, "MVP\x00".as_ptr() as *const i8)
        };

        // Projection matrix : 45° Field of View, 4:3 ratio, display range : 0.1 unit <-> 100 units
        //let projection : tutcommon::Matrix4f = tutcommon::Matrix4f::perspective(45.0
        //, 4.0 / 3.0
        //, 0.1
        //, 100.0);

        let projection: Matrix4f = Matrix4f::ortho(-10.0, 10.0, -10.0, 10.0, 0.0, 100.0);

        println!("Projection matrix: {:?}", projection);

        // Camera matrix
        let view = Matrix4f::look_at(
            &Vector3f(4.0, 3.0, 3.0), // Camera is at (4,3,3), in World Space
            &Vector3f(0.0, 0.0, 0.0), // and looks at the origin
            &Vector3f(0.0, 1.0, 0.0) // Head is up (set to 0,-1,0 to look upside-down)
        );

        println!("View matrix: {:?}", view);

        // Model matrix : an identity matrix (model will be at the origin)
        // or can use std::default::Default::default();
        let model = Matrix4f::rotate(45.0, Vector3f(10.0, 1.0, -2.0));

        println!("Model matrix: {:?}", model);

        // Our ModelViewProjection : multiplication of our 3 matrices
        // Remember, matrix multiplication is the other way around
        let mvp = projection.mul(&view).mul(&model);

        println!("MVP matrix: {:?}", mvp);

        unsafe {
            // Generate 1 buffer.
            gl::GenBuffers(1, &mut vertex_buffer_id);

            // Set it current.
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_id);

            // Send vertices to buffer.
            gl::BufferData(gl::ARRAY_BUFFER,
                           std::mem::size_of_val(&G_VERTEX_BUFFER_DATA) as isize,
                           std::mem::transmute(&G_VERTEX_BUFFER_DATA),
                           gl::STATIC_DRAW);
        }

        GLScene {
            vertex_array_id: vertex_array_id,
            vertex_buffer_id: vertex_buffer_id,
            program_id: program_id,
            matrix_id: matrix_id,
            mvp: mvp,
        }
    }

    #[doc = "Render scene each frame."]
    pub fn draw(&self) {

        unsafe {
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

            // Use our shader
            gl::UseProgram(self.program_id);

            // Send our transformation to the currently bound shader,
            // in the "MVP" uniform.
            gl::UniformMatrix4fv(self.matrix_id, 1, gl::FALSE, self.mvp.as_ptr());

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
