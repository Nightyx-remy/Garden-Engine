use std::collections::HashMap;
use std::ffi::CString;
use std::ptr;
use cgmath::{Matrix, Matrix4, Vector2, Vector3};
use gl::types::{GLchar, GLenum, GLint, GLsizei, GLuint};

use crate::{error, info};

use super::Color;

////////////////////////////////////////////////////////////////////////////////////////////////////
//                                             Shader                                             //
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Shader {
    path: String,
    program_id: GLuint,
    vertex_id: GLuint,
    fragment_id: GLuint,
    uniforms: HashMap<String, GLint>,
}

impl Shader {

    pub fn load<Str: ToString>(path: Str) -> Shader {
        unsafe {
            // Compile the vertex & fragment shader
            let vertex_id = Self::compile_shader(format!("{}/VS.glsl", path.to_string()), gl::VERTEX_SHADER);
            let fragment_id = Self::compile_shader(format!("{}/FS.glsl", path.to_string()), gl::FRAGMENT_SHADER);

            // Create the program & Attach the shaders
            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vertex_id);
            gl::AttachShader(program_id, fragment_id);

            // Link the program
            gl::LinkProgram(program_id);
            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
                let mut info_log = Vec::<u8>::with_capacity(len as usize);
                gl::GetProgramInfoLog(program_id, info_log.capacity() as i32, &mut len, info_log.as_mut_ptr() as *mut GLchar);
                info_log.set_len(len as usize);
                error!("Garlic", "Failed to validate shader '{}'\n{}", path.to_string(), std::str::from_utf8(&info_log).unwrap());
            }

            // Validate the program
            gl::ValidateProgram(program_id);
            success = gl::FALSE as GLint;
            gl::GetProgramiv(program_id, gl::VALIDATE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                let mut len = 0;
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
                let mut info_log = Vec::<u8>::with_capacity(len as usize);
                gl::GetProgramInfoLog(program_id, info_log.capacity() as i32, &mut len, info_log.as_mut_ptr() as *mut GLchar);
                info_log.set_len(len as usize);
                error!("Garlic", "Failed to link shader '{}'\n{}", path.to_string(), std::str::from_utf8(&info_log).unwrap());
            }

            info!("Garlic", "Shader '{}' loaded!", path.to_string());

            return Shader {
                path: path.to_string(),
                program_id,
                vertex_id,
                fragment_id,
                uniforms: HashMap::new(),
            }
        }
    }

    unsafe fn compile_shader(path: String, shader: GLenum) -> GLuint {
        let id = gl::CreateShader(shader);

        let src = match std::fs::read_to_string(path.clone()) {
            Ok(res) => res,
            Err(err) => {
                error!("Garlic", "Failed to read shader '{}', {}", path, err);
                "".to_string()
            }
        };
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(id, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(id);

        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            let mut len = 0;
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            let mut info_log = Vec::<u8>::with_capacity(len as usize);
            gl::GetShaderInfoLog(id, info_log.capacity() as i32, &mut len, info_log.as_mut_ptr() as *mut GLchar);
            info_log.set_len(len as usize);
            error!("Garlic", "Failed to compile shader '{}'\n{}", path, std::str::from_utf8(&info_log).unwrap());
        }

        return id;
    }

    pub fn dispose(&self) {
        unsafe {
            gl::DetachShader(self.program_id, self.vertex_id);
            gl::DetachShader(self.program_id, self.fragment_id);
            gl::DeleteShader(self.vertex_id);
            gl::DeleteShader(self.fragment_id);
            gl::DeleteProgram(self.program_id);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::UseProgram(0);
        }
    }

    unsafe fn get_uniform_location(&mut self, name: String) -> GLint {
        return if let Some(uniform) = self.uniforms.get(&name) {
            *uniform
        } else {
            let name_c_str = CString::new(name.clone()).unwrap();
            let location = gl::GetUniformLocation(self.program_id, name_c_str.as_ptr());
            if location == -1 {
                error!("Garlic", "Failed to get uniform '{}' in '{}'!", name, self.path);
            }
            self.uniforms.insert(name, location);
            location
        }
    }

    pub fn load_i32<Str: ToString>(&mut self, name: Str, value: i32) {
        unsafe {
            let location = self.get_uniform_location(name.to_string());
            gl::Uniform1i(location, value);
        }
    }

    pub fn load_i32_arr<Str: ToString, const N: usize>(&mut self, name: Str, arr: [i32; N]) {
        unsafe {
            let location = self.get_uniform_location(name.to_string());
            gl::Uniform1iv(location, N as GLsizei, arr.as_ptr());
        }
    }

    pub fn load_f32<Str: ToString>(&mut self, name: Str, value: f32) {
        unsafe {
            let location = self.get_uniform_location(name.to_string());
            gl::Uniform1f(location, value);
        }
    }

    pub fn load_f32_arr<Str: ToString, const N: usize>(&mut self, name: Str, arr: [f32; N]) {
        unsafe {
            let location = self.get_uniform_location(name.to_string());
            gl::Uniform1fv(location, N as GLsizei, arr.as_ptr());
        }
    }

    pub fn load_vec2_i32<Str: ToString>(&mut self, name: Str, value: Vector2<i32>) {
        unsafe {
            let location = self.get_uniform_location(name.to_string());
            gl::Uniform2i(location, value.x, value.y);
        }
    }

    pub fn load_vec2_i32_arr<Str: ToString, const N: usize>(&mut self, name: Str, arr: [Vector2<i32>; N]) {
        unsafe {
            let mut vec = Vec::new();
            for element in arr {
                vec.push(element.x);
                vec.push(element.y);
            }
            let location = self.get_uniform_location(name.to_string());
            gl::Uniform2iv(location, N as GLsizei, vec.as_ptr());
        }
    }

    pub fn load_vec2_f32<Str: ToString>(&mut self, name: Str, value: Vector2<f32>) {
        unsafe {
            let location = self.get_uniform_location(name.to_string());
            gl::Uniform2f(location, value.x, value.y);
        }
    }

    pub fn load_vec2_f32_arr<Str: ToString, const N: usize>(&mut self, name: Str, arr: [Vector2<f32>; N]) {
        unsafe {
            let mut vec = Vec::new();
            for element in arr {
                vec.push(element.x);
                vec.push(element.y);
            }
            let location = self.get_uniform_location(name.to_string());
            gl::Uniform2fv(location, N as GLsizei, vec.as_ptr());
        }
    }

    pub fn load_vec3_i32<Str: ToString>(&mut self, name: Str, value: Vector3<i32>) {
        unsafe {
            let location = self.get_uniform_location(name.to_string());
            gl::Uniform3i(location, value.x, value.y, value.z);
        }
    }

    pub fn load_vec3_i32_arr<Str: ToString, const N: usize>(&mut self, name: Str, arr: [Vector3<i32>; N]) {
        unsafe {
            let mut vec = Vec::new();
            for element in arr {
                vec.push(element.x);
                vec.push(element.y);
                vec.push(element.z);
            }
            let location = self.get_uniform_location(name.to_string());
            gl::Uniform3iv(location, N as GLsizei, vec.as_ptr());
        }
    }

    pub fn load_vec3_f32<Str: ToString>(&mut self, name: Str, value: Vector3<f32>) {
        unsafe {
            let location = self.get_uniform_location(name.to_string());
            gl::Uniform3f(location, value.x, value.y, value.z);
        }
    }

    pub fn load_vec3_f32_arr<Str: ToString, const N: usize>(&mut self, name: Str, arr: [Vector3<f32>; N]) {
        unsafe {
            let mut vec = Vec::new();
            for element in arr {
                vec.push(element.x);
                vec.push(element.y);
                vec.push(element.z);
            }
            let location = self.get_uniform_location(name.to_string());
            gl::Uniform3fv(location, N as GLsizei, vec.as_ptr());
        }
    }


    pub fn load_color<Str: ToString>(&mut self, name: Str, value: Color) {
        unsafe {
            let location = self.get_uniform_location(name.to_string());
            gl::Uniform4f(location, value.red, value.green, value.blue, value.alpha);
        }
    }

    pub fn load_color_arr<Str: ToString, const N: usize>(&mut self, name: Str, arr: [Color; N]) {
        unsafe {
            let mut vec = Vec::new();
            for element in arr {
                vec.push(element.red);
                vec.push(element.green);
                vec.push(element.blue);
                vec.push(element.alpha);
            }
            let location = self.get_uniform_location(name.to_string());
            gl::Uniform4fv(location, N as GLsizei, vec.as_ptr());
        }
    }

    pub fn load_mat4<Str: ToString>(&mut self, name: Str, matrix: Matrix4<f32>) {
        unsafe {
            let location = self.get_uniform_location(name.to_string());
            gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
        }
    }



}