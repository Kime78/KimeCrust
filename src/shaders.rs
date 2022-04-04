use std::{fs, ptr::null, slice::{from_raw_parts, self}, str::from_utf8};

use gl::{types::*};
use glm::*;

#[derive(PartialEq, Eq)]
pub enum ShaderType {
    Vertex,
    Fragment
}

#[derive(Debug)]
pub struct Shader {
    id: GLuint
}
pub struct ShaderProgram {
    id: GLuint,
    vertex_id : GLuint,
    fragment_id : GLuint
}

impl Shader {
    pub fn new(mut src: String, typ: ShaderType) -> Self {
        let mut shader_id :u32 = 0;
        
        if typ == ShaderType::Vertex {
            shader_id = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        }
        else if typ == ShaderType::Fragment {
            shader_id = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };
        }
        else if shader_id == 0 {
            panic!("The type of shader is incorrect!");
        }

        src.push('\0');

        unsafe {
            let why = src.as_ptr() as *const GLchar;
            let just_why = &why as *const *const GLchar;
            gl::ShaderSource(shader_id, 1, just_why, null());
            gl::CompileShader(shader_id);

            let mut compiled: i32 = -1;
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut compiled as *mut i32);
            if compiled == 0 {
                let mut error_lenght: i32 = 0;
                gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut error_lenght as *mut i32);
                let mut error = Vec::<GLchar>::with_capacity(error_lenght as usize);
                gl::GetShaderInfoLog(shader_id, error_lenght, &mut error_lenght as *mut i32, error.as_mut_ptr());
                gl::DeleteShader(shader_id);

                let error_str = from_raw_parts(error.as_ptr(), error_lenght as usize);
                let u8slice : &[u8] = slice::from_raw_parts(error_str.as_ptr() as *const u8, error_str.len());
                let print_str = from_utf8(u8slice).unwrap();
                println!("{}", print_str);
                panic!();
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
    pub fn new(vertex: Shader, fragment: Shader) -> Self{
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
            }
        }
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

    pub fn upload_vec2(&self, vector: Vec2, var: String) {
        unsafe {
            let location = gl::GetUniformLocation(self.id, var.as_ptr() as *const i8);
            gl::Uniform2f(location, vector.x, vector.y);
        }
    }

    pub fn upload_vec3(&self, vector: Vec3, var: String) {
        unsafe {
            let location = gl::GetUniformLocation(self.id, var.as_ptr() as *const i8);
            gl::Uniform3f(location, vector.x, vector.y, vector.z);
        }
    }

    pub fn upload_vec4(&self, vector: Vec4, var: String) {
        unsafe {
            let location = gl::GetUniformLocation(self.id, var.as_ptr() as *const i8);
            gl::Uniform4f(location, vector.x, vector.y, vector.z, vector.w);
        }
    }

    pub fn upload_ivec2(&self, vector: IVec2, var: String) {
        unsafe {
            let location = gl::GetUniformLocation(self.id, var.as_ptr() as *const i8);
            gl::Uniform2i(location, vector.x, vector.y);
        }
    }

    pub fn upload_ivec3(&self, vector: IVec3, var: String) {
        unsafe {
            let location = gl::GetUniformLocation(self.id, var.as_ptr() as *const i8);
            gl::Uniform3i(location, vector.x, vector.y, vector.z);
        }
    }

    pub fn upload_ivec4(&self, vector: IVec4, var: String) {
        unsafe {
            let location = gl::GetUniformLocation(self.id, var.as_ptr() as *const i8);
            gl::Uniform4i(location, vector.x, vector.y, vector.z, vector.w);
        }
    }

    pub fn upload_f32(&self, val: f32, var: String) {
        unsafe {
            let location = gl::GetUniformLocation(self.id, var.as_ptr() as *const i8);
            gl::Uniform1f(location, val);
        }
    }

    pub fn upload_u32(&self, val: u32, var: String) {
        unsafe {
            let location = gl::GetUniformLocation(self.id, var.as_ptr() as *const i8);
            gl::Uniform1ui(location, val);
        }
    }
    
    pub fn upload_i32(&self, val: i32, var: String) {
        unsafe {
            let location = gl::GetUniformLocation(self.id, var.as_ptr() as *const i8);
            gl::Uniform1i(location, val);
        }
    }

    pub fn upload_i32_array(&self, arr: Vec<i32>, var: String) {
        unsafe {
            let location = gl::GetUniformLocation(self.id, var.as_ptr() as *const i8);
            gl::Uniform1iv(location, arr.len() as i32, arr.as_ptr())
        }
    } 
    
    //TODO MATRIX BS
    pub fn upload_mat3(&self, matrix: Mat3, var: String) {
        unsafe {
            let location = gl::GetUniformLocation(self.id, var.as_ptr() as *const i8);
            //gl::Uniform4f(location, vector.x, vector.y, vector.z, vector.w);

            let test = matrix.as_array();
            let mut mat: [f32; 9] = [0.0; 9];
            let mut k = 0;

            for e in test {
                mat[k] = e.x;
                mat[k + 1] = e.y;
                mat[k + 2] = e.z;
                k += 3;
            }

            let pmat = mat.as_ptr();
            gl::UniformMatrix3fv(location, 1, 0, pmat);
        }
    }
    
    pub fn upload_mat4(&self, matrix: Mat4, var: String) {
        unsafe {
            let location = gl::GetUniformLocation(self.id, var.as_ptr() as *const i8);
            //gl::Uniform4f(location, vector.x, vector.y, vector.z, vector.w);

            let test = matrix.as_array();
            let mut mat: [f32; 16] = [0.0; 16];
            let mut k = 0;

            for e in test {
                mat[k] = e.x;
                mat[k + 1] = e.y;
                mat[k + 2] = e.z;
                mat[k + 3] = e.w;
                k += 4;
            }

            let pmat = mat.as_ptr();
            println!("{pmat:?}");
            gl::UniformMatrix3fv(location, 1, 0, pmat);
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}