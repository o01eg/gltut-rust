use std;

use gl;
use gl::types::{GLfloat, GLint, GLuint};

use libc::types::common::c95::c_void;

use tutcommon;

// Our vertices. Tree consecutive floats give a 3D vertex; Three consecutive vertices give a triangle.
// A cube has 6 faces with 2 triangles each, so this makes 6*2=12 triangles, and 12*3 vertices
static G_VERTEX_BUFFER_DATA : [GLfloat; 12*3*3] = [
    -1.0,-1.0,-1.0, // triangle 1 : begin
    -1.0,-1.0, 1.0,
    -1.0, 1.0, 1.0, // triangle 1 : end
    1.0, 1.0,-1.0, // triangle 2 : begin
    -1.0,-1.0,-1.0,
    -1.0, 1.0,-1.0, // triangle 2 : end
    1.0,-1.0, 1.0,
    -1.0,-1.0,-1.0,
    1.0,-1.0,-1.0,
    1.0, 1.0,-1.0,
    1.0,-1.0,-1.0,
    -1.0,-1.0,-1.0,
    -1.0,-1.0,-1.0,
    -1.0, 1.0, 1.0,
    -1.0, 1.0,-1.0,
    1.0,-1.0, 1.0,
    -1.0,-1.0, 1.0,
    -1.0,-1.0,-1.0,
    -1.0, 1.0, 1.0,
    -1.0,-1.0, 1.0,
    1.0,-1.0, 1.0,
    1.0, 1.0, 1.0,
    1.0,-1.0,-1.0,
    1.0, 1.0,-1.0,
    1.0,-1.0,-1.0,
    1.0, 1.0, 1.0,
    1.0,-1.0, 1.0,
    1.0, 1.0, 1.0,
    1.0, 1.0,-1.0,
    -1.0, 1.0,-1.0,
    1.0, 1.0, 1.0,
    -1.0, 1.0,-1.0,
    -1.0, 1.0, 1.0,
    1.0, 1.0, 1.0,
    -1.0, 1.0, 1.0,
    1.0,-1.0, 1.0
];

// Two UV coordinatesfor each vertex. They were created with Blender. You'll learn shortly how to do this yourself.
static G_UV_BUFFER_DATA : [GLfloat; 12*3*2] = [
    0.000059, 1.0-0.000004,
    0.000103, 1.0-0.336048,
    0.335973, 1.0-0.335903,
    1.000023, 1.0-0.000013,
    0.667979, 1.0-0.335851,
    0.999958, 1.0-0.336064,
    0.667979, 1.0-0.335851,
    0.336024, 1.0-0.671877,
    0.667969, 1.0-0.671889,
    1.000023, 1.0-0.000013,
    0.668104, 1.0-0.000013,
    0.667979, 1.0-0.335851,
    0.000059, 1.0-0.000004,
    0.335973, 1.0-0.335903,
    0.336098, 1.0-0.000071,
    0.667979, 1.0-0.335851,
    0.335973, 1.0-0.335903,
    0.336024, 1.0-0.671877,
    1.000004, 1.0-0.671847,
    0.999958, 1.0-0.336064,
    0.667979, 1.0-0.335851,
    0.668104, 1.0-0.000013,
    0.335973, 1.0-0.335903,
    0.667979, 1.0-0.335851,
    0.335973, 1.0-0.335903,
    0.668104, 1.0-0.000013,
    0.336098, 1.0-0.000071,
    0.000103, 1.0-0.336048,
    0.000004, 1.0-0.671870,
    0.336024, 1.0-0.671877,
    0.000103, 1.0-0.336048,
    0.336024, 1.0-0.671877,
    0.335973, 1.0-0.335903,
    0.667969, 1.0-0.671889,
    1.000004, 1.0-0.671847,
    0.667979, 1.0-0.335851
];

#[doc = "Moved out drawing GL stuff to avoid mess with the other code."]
pub struct GLScene {
    vertex_array_id : GLuint, //VAO id.
    vertex_buffer_id : GLuint, //VBO id.
    uv_buffer_id : GLuint, // UV id.
    program_id : GLuint, //Shader program id.
    texture_id : GLuint, // Texture id.
    matrix_uniform_id : GLint, // MVP uniform locaion.
    texture_uniform_id : GLint, // myTextureSampler uniform location.
    mvp : tutcommon::Matrix4f, // Matrix 
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
        let program_id = tutcommon::glutils::load_program("data/tut05/TransformVertexShader.vertexshader"
            , "data/tut05/TextureFragmentShader.fragmentshader");

        let matrix_uniform_id = unsafe {
            // Get a handle for our "MVP" uniform 
            gl::GetUniformLocation(program_id, "MVP\x00".as_ptr() as * const i8)
        };

        let texture_uniform_id = unsafe {
            gl::GetUniformLocation(program_id, "myTextureSampler\x00".as_ptr() as * const i8)
        };

        // Projection matrix : 45° Field of View, 4:3 ratio, display range : 0.1 unit <-> 100 units
        let projection : tutcommon::Matrix4f = tutcommon::Matrix4f::perspective(45.0, 4.0 / 3.0, 0.1, 100.0);
        
        // Camera matrix        
        let view = tutcommon::Matrix4f::look_at(
            tutcommon::Vector3f(4.0, 3.0, 3.0), // Camera is at (4,3,3), in World Space
            tutcommon::Vector3f(0.0, 0.0, 0.0), // and looks at the origin
            tutcommon::Vector3f(0.0, 1.0, 0.0) // Head is up (set to 0,-1,0 to look upside-down)
        );

        // Model matrix : an identity matrix (model will be at the origin)
        let model = std::default::Default::default();

        // Our ModelViewProjection : multiplication of our 3 matrices        
        let mvp = projection.mul(&view).mul(&model); // Remember, matrix multiplication is the other way around

        let mut vertex_buffer_id = 0;
        
        unsafe {
            // Generate 1 buffer.
            gl::GenBuffers(1, &mut vertex_buffer_id);

            // Set it current.
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_id);

            // Send vertices to buffer.
            gl::BufferData(gl::ARRAY_BUFFER
                , std::mem::size_of_val(&G_VERTEX_BUFFER_DATA) as i64
                , std::mem::transmute(&G_VERTEX_BUFFER_DATA)
                , gl::STATIC_DRAW);
        }

        let mut uv_buffer_id = 0;

        unsafe {
            gl::GenBuffers(1, &mut uv_buffer_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, uv_buffer_id);
            gl::BufferData(gl::ARRAY_BUFFER
                , std::mem::size_of_val(&G_UV_BUFFER_DATA) as i64
                , std::mem::transmute(&G_UV_BUFFER_DATA)
                , gl::STATIC_DRAW);
        }

        let texture_id = GLScene::load_texture("data/tut05/uvtemplate.bmp");

        GLScene { vertex_array_id : vertex_array_id
            , vertex_buffer_id : vertex_buffer_id
            , uv_buffer_id : uv_buffer_id
            , texture_id : texture_id
            , program_id : program_id
            , matrix_uniform_id : matrix_uniform_id
            , texture_uniform_id : texture_uniform_id
            , mvp : mvp }
    }

    #[doc = "Update data each frame."]
    pub fn update(&mut self) {
    }

    #[doc = "Render scene each frame."]
    pub fn draw(&self) {
        unsafe {
            // Use our shader
            gl::UseProgram(self.program_id);

            // Send our transformation to the currently bound shader,
            // in the "MVP" uniform.
            gl::UniformMatrix4fv(self.matrix_uniform_id, 1, gl::FALSE, self.mvp.as_ptr());

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
            // Set our "myTextureSampler" sampler to user Texture Unit 0
            gl::Uniform1i(self.texture_uniform_id, 0);

            //1st attribute buffer : vertices
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_id);
            gl::VertexAttribPointer(
                0, // attribute 0. No particular reason for 0, but must match the layout in the shader.
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
                1, // attribute 1. No particular reason for 1, but must match the layout in the shader.
                2, // size
                gl::FLOAT, // type
                gl::FALSE, // normalized?
                0, // stride
                std::ptr::null() // array buffer offset
            );

            // Draw the triangle!
            // 12*3 indices starting at 0 -> 12 triangles -> 6 squares
            gl::DrawArrays(gl::TRIANGLES, 0, 12*3);

            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
        }
    }

    fn load_texture(file:&str) -> GLuint {
        let mut surface = tutcommon::load_bmp(file);

        let (width, height) = surface.get_size();

        surface.get_pixel_format();

        let mut texture_id = 0;
        surface.with_lock(|data| {
            unsafe {
                gl::GenTextures(1, &mut texture_id);
                gl::BindTexture(gl::TEXTURE_2D, texture_id);

                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, width as i32, height as i32, 0, gl::BGR, gl::UNSIGNED_BYTE, data.as_ptr() as * const c_void);

                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            }
        });

        texture_id
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

