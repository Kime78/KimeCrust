use std::{fs, ptr::null, slice::{from_raw_parts, self}, str::from_utf8};

use gl::{types::*};

#[derive(PartialEq, Eq)]
pub enum ShaderType {
    Vertex,
    Fragment
}

pub struct Shader {
    id: GLuint
}
pub struct ShaderProgram {
    id: GLuint,
    vertex_id : GLuint,
    fragment_id : GLuint
}

impl Shader {
    pub fn new(path: String, typ: ShaderType) -> Self {
        let mut shader_id :u32 = 0;
        let data = fs::read_to_string(path).unwrap();

        if typ == ShaderType::Vertex {
            shader_id = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        }
        else if typ == ShaderType::Fragment {
            shader_id = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
        }
        else if shader_id == 0 {
            panic!("The type of shader is incorrect!");
        }
        
        unsafe {
            let why = data.as_ptr() as *const GLchar;
            let just_why = &why as *const *const GLchar;
            gl::ShaderSource(shader_id, 1, just_why, null());
            gl::CompileShader(shader_id);

            let mut compiled: i32 = -1;
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut compiled as *mut i32);
            if compiled == 0 {
                let mut error_lenght = 0;
                let mut error: Vec<GLchar> = Vec::new();
                gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut error_lenght as *mut i32);
                gl::GetShaderInfoLog(shader_id, error_lenght, &mut error_lenght as *mut i32, error.as_mut_ptr());
                gl::DeleteShader(shader_id);

                let error_str = from_raw_parts(error.as_ptr(), error_lenght as usize);
                let u8slice : &[u8] = slice::from_raw_parts(error_str.as_ptr() as *const u8, error_str.len());
                let print_str = from_utf8(u8slice).unwrap();
                panic!("{}", print_str);
            }
        };
        
        Self { id: shader_id }
    }

    pub fn get_shader_id(&self) -> GLuint {
        return self.id;
    }

    //Deletes the shader
    pub fn delete(&self) {
        unsafe { gl::DeleteShader(self.id); }
    }
}

impl ShaderProgram {
    pub fn new(vertex: Shader, fragment: Shader) {
        unsafe {
            let program = gl::CreateProgram();

            gl::AttachShader(program, vertex.get_shader_id());
            gl::AttachShader(program, fragment.get_shader_id());

            gl::LinkProgram(program);

            let mut compiled = -1;
            gl::GetProgramiv(program, gl::COMPILE_STATUS, &mut compiled as *mut i32);
            if compiled == 0 {
                let mut error_lenght = 0;
                let mut error: Vec<GLchar> = Vec::new();
                gl::GetShaderiv(program, gl::INFO_LOG_LENGTH, &mut error_lenght as *mut i32);
                gl::GetShaderInfoLog(program, error_lenght, &mut error_lenght as *mut i32, error.as_mut_ptr());
                gl::DeleteShader(program);

                let error_str = from_raw_parts(error.as_ptr(), error_lenght as usize);
                let u8slice : &[u8] = slice::from_raw_parts(error_str.as_ptr() as *const u8, error_str.len());
                let print_str = from_utf8(u8slice).unwrap();
                panic!("{}", print_str);
            }
            Self {
                id: program,
                vertex_id: vertex.get_shader_id(),
                fragment_id: fragment.get_shader_id()
            };
        };
    }

    pub fn get_program_id(&self) -> GLuint {
        return self.id;
    }

    pub fn delete(&self) {
        unsafe {
            gl::DetachShader(self.id, self.vertex_id);
            gl::DetachShader(self.id, self.fragment_id);
            gl::DeleteProgram(self.id);
        }
    }

    pub fn update_shader_program(&self, new_vertex: Shader, new_fragment: Shader) -> Self {
        unsafe {
            gl::DetachShader(self.id, self.vertex_id);
            gl::DetachShader(self.id, self.fragment_id);

            gl::AttachShader(self.id, new_vertex.get_shader_id());
            gl::AttachShader(self.id, new_fragment.get_shader_id());

            gl::LinkProgram(self.id);

            let mut compiled = -1;
            gl::GetProgramiv(self.id, gl::COMPILE_STATUS, &mut compiled as *mut i32);
            if compiled == 0 {
                let mut error_lenght = 0;
                let mut error: Vec<GLchar> = Vec::new();
                gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut error_lenght as *mut i32);
                gl::GetShaderInfoLog(self.id, error_lenght, &mut error_lenght as *mut i32, error.as_mut_ptr());
                gl::DeleteShader(self.id);

                let error_str = from_raw_parts(error.as_ptr(), error_lenght as usize);
                let u8slice : &[u8] = slice::from_raw_parts(error_str.as_ptr() as *const u8, error_str.len());
                let print_str = from_utf8(u8slice).unwrap();
                panic!("{}", print_str);
            }
            Self {
                id: self.id,
                vertex_id: new_vertex.get_shader_id(),
                fragment_id: new_fragment.get_shader_id()
            }   
        }
    }
}