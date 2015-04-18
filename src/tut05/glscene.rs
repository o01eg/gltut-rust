use std;

use gl;
use gl::types::{GLchar, GLfloat, GLint, GLuint};

use rand;
use rand::Rng;

use tutcommon;

// An array of 3 vectors which represents 3 vertices.
static G_TRIANGLE_VERTEX_BUFFER_DATA : [GLfloat; 9] = [
    -1.0, -1.0, 0.0,
    1.0, -1.0, 0.0,
    0.0, 1.0, 0.0
];

// One color for each vertex. They were generated randomly.
static G_TRIANGLE_COLOR_BUFFER_DATA : [GLfloat; 9] = [
    0.583,  0.771,  0.014,
    0.609,  0.115,  0.436,
    0.327,  0.483,  0.844
];

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

#[doc = "Moved out drawing GL stuff to avoid mess with the other code."]
pub struct GLScene {
    vertex_array_id : GLuint, //VAO id.
    vertex_buffer_id : GLuint, //VBO id.
    tri_vertex_buffer_id : GLuint,
    color_buffer_data : Vec<GLfloat>, // Color buffer 12 * 3 * 3
    color_buffer_id : GLuint, // Color buffer id.
    tri_color_buffer_id : GLuint,
    program_id : GLuint, //Shader program id.
    matrix_id : GLint, // MVP uniform locaion.
    mvp : tutcommon::Matrix4f, // Matrix 
    tri_mvp : tutcommon::Matrix4f, // Matrix
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
        let program_id = GLScene::load_program("data/tut05/TransformVertexShader.vertexshader"
            , "data/tut05/ColorFragmentShader.fragmentshader");

        let matrix_id = unsafe {
            // Get a handle for our "MVP" uniform 
            gl::GetUniformLocation(program_id, "MVP".as_ptr() as * const i8)
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

        let tri_model = tutcommon::Matrix4f::translate(tutcommon::Vector3f(1.5, 1.0, -0.5));
        let tri_mvp = projection.mul(&view).mul(&tri_model);

        let mut vertex_buffer_id = 0;
        let mut tri_vertex_buffer_id = 0;
        
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

        unsafe {
            // Generate 1 buffer.
            gl::GenBuffers(1, &mut tri_vertex_buffer_id);

            // Set it current.
            gl::BindBuffer(gl::ARRAY_BUFFER, tri_vertex_buffer_id);

            // Send vertices to buffer.
            gl::BufferData(gl::ARRAY_BUFFER
                , std::mem::size_of_val(&G_TRIANGLE_VERTEX_BUFFER_DATA) as i64
                , std::mem::transmute(&G_TRIANGLE_VERTEX_BUFFER_DATA)
                , gl::STATIC_DRAW);
        }

        let mut color_buffer_id = 0;
        let mut tri_color_buffer_id = 0;
        let mut color_buffer_data = Vec::with_capacity(12 * 3 * 3);
        let mut rng = rand::thread_rng();
        for _ in 0 .. 12 * 3 * 3 {
            color_buffer_data.push(rng.next_f32());
        }

        unsafe {
            gl::GenBuffers(1, &mut color_buffer_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, color_buffer_id);
            gl::BufferData(gl::ARRAY_BUFFER
                , (std::mem::size_of::<GLfloat>() * color_buffer_data.len()) as i64
                , std::mem::transmute(color_buffer_data.as_ptr())
                , gl::STATIC_DRAW);
        }

        unsafe {
            gl::GenBuffers(1, &mut tri_color_buffer_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, tri_color_buffer_id);
            gl::BufferData(gl::ARRAY_BUFFER
                , std::mem::size_of_val(&G_TRIANGLE_COLOR_BUFFER_DATA) as i64
                , std::mem::transmute(&G_TRIANGLE_COLOR_BUFFER_DATA)
                , gl::STATIC_DRAW);
        }
       
        GLScene { vertex_array_id : vertex_array_id
            , vertex_buffer_id : vertex_buffer_id
            , tri_vertex_buffer_id : tri_vertex_buffer_id
            , color_buffer_data : color_buffer_data
            , color_buffer_id : color_buffer_id
            , tri_color_buffer_id : tri_color_buffer_id
            , program_id : program_id
            , matrix_id : matrix_id
            , mvp : mvp
            , tri_mvp : tri_mvp }
    }

    #[doc = "Update data each frame."]
    pub fn update(&mut self) {
        // change color each frame
        self.color_buffer_data.truncate(0);
        self.color_buffer_data.reserve(12 * 3 * 3);
        let mut rng = rand::thread_rng();
        for _ in 0 .. 12 * 3 * 3 {
            self.color_buffer_data.push(rng.next_f32());
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.color_buffer_id);
            gl::BufferData(gl::ARRAY_BUFFER
                , (std::mem::size_of::<GLfloat>() * self.color_buffer_data.len()) as i64
                , std::mem::transmute(self.color_buffer_data.as_ptr())
                , gl::STATIC_DRAW);
        }
    }

    #[doc = "Render scene each frame."]
    pub fn draw(&self) {
        unsafe {
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
            gl::BindBuffer(gl::ARRAY_BUFFER, self.color_buffer_id);
            gl::VertexAttribPointer(
                1, // attribute 1. No particular reason for 1, but must match the layout in the shader.
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
            // 12*3 indices starting at 0 -> 12 triangles -> 6 squares
            gl::DrawArrays(gl::TRIANGLES, 0, 12*3);

            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
        }

        unsafe {
            //1st attribute buffer : vertices
            gl::EnableVertexAttribArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.tri_vertex_buffer_id);
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
            gl::BindBuffer(gl::ARRAY_BUFFER, self.tri_color_buffer_id);
            gl::VertexAttribPointer(
                1, // attribute 1. No particular reason for 1, but must match the layout in the shader.
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
            gl::UniformMatrix4fv(self.matrix_id, 1, gl::FALSE, self.tri_mvp.as_ptr());

            // Draw the triangle!
            // 12*3 indices starting at 0 -> 12 triangles -> 6 squares
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
        }
    }

    fn load_program(vertex_file_path : &str, fragment_file_path : &str) -> GLuint {
        unsafe {
            // Create the shaders.
            let vertex_shader_id = gl::CreateShader(gl::VERTEX_SHADER);
            let fragment_shader_id = gl::CreateShader(gl::FRAGMENT_SHADER);
        
            // Read the Vertex Shader code from the file.
            let vertex_shader_code = tutcommon::read_source_from_file(vertex_file_path);
        
            // Read the Fragment Shader code from the file
            let fragment_shader_code = tutcommon::read_source_from_file(fragment_file_path);

            // Compile Vertex Shader
            println!("Compiling shader: {}", vertex_file_path);
            GLScene::compile_and_check_shader(vertex_shader_id, &vertex_shader_code);

            // Compile Fragment Shader
            println!("Compiling shader: {}", fragment_file_path);
            GLScene::compile_and_check_shader(fragment_shader_id, &fragment_shader_code);

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

    fn compile_and_check_shader(shader_id : GLuint, shader_source : &str) {
        let mut result = 0;
        let mut info_log_length = 0;
        unsafe {
            let source : &[i8] = std::mem::transmute(shader_source);
            // Compile Shader
            gl::ShaderSource(shader_id, 1, std::mem::transmute(&source), std::ptr::null());
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
}

#[doc = "Always clean up after yourself."]
impl Drop for GLScene {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);

            gl::DeleteBuffers(1, &self.vertex_buffer_id);
            gl::DeleteBuffers(1, &self.tri_vertex_buffer_id);
            gl::DeleteBuffers(1, &self.color_buffer_id);
            gl::DeleteBuffers(1, &self.tri_color_buffer_id);

            gl::DeleteVertexArrays(1, &self.vertex_array_id);
        }
    }
}

