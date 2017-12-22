use gl;
use gl::types::*;
use std;
use std::ffi::CString;

use shader::Shader;

pub struct ShaderProgram {
    pub gl_id: GLuint,
}

#[derive(Debug)]
pub struct ShaderProgramError {
    pub message: String,
    pub info_log: Option<String>,
}

impl ShaderProgram {
    pub fn link(shaders: &[&Shader]) -> Result<Self, ShaderProgramError> {
        let gl_id = unsafe {
            let gl_id = gl::CreateProgram();
            if gl_id == 0 {
                return Err(ShaderProgramError {
                    message: "Failed to create GPU shader program".into(),
                    info_log: None,
                });
            }
            for shader in shaders {
                gl::AttachShader(gl_id, shader.gl_id);
            }
            gl::LinkProgram(gl_id);
            for shader in shaders {
                gl::DetachShader(gl_id, shader.gl_id);
            }
            let mut is_linked = 0;
            gl::GetProgramiv(gl_id, gl::LINK_STATUS, &mut is_linked);
            if is_linked == 0 {
                let mut log_len = 0;
                gl::GetProgramiv(gl_id, gl::INFO_LOG_LENGTH, &mut log_len);
                let mut log_buf = vec![0; log_len as usize];
                gl::GetProgramInfoLog(
                    gl_id,
                    log_len,
                    std::ptr::null_mut(),
                    log_buf.as_mut_ptr() as *mut _,
                    );
                return Err(ShaderProgramError {
                    message: "Failed to link shader program".into(),
                    info_log: String::from_utf8(log_buf).ok(),
                });
            }
            gl_id
        };
        Ok(ShaderProgram {
            gl_id: gl_id,
        })
    }

    pub fn get_uniform_location(&self, uniform: &str) -> Option<u32> {
        unsafe {
            if let Ok(c_name) = CString::new(uniform) {
                let gl_location = gl::GetUniformLocation(self.gl_id, c_name.as_ptr() as *const _);
                if gl_location == -1 {
                    None
                } else {
                    Some(gl_location as _)
                }
            } else {
                None
            }
        }
    }

    pub fn activate(&mut self) {
        unsafe {
            gl::UseProgram(self.gl_id);
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.gl_id);
        }
        self.gl_id = 0;
    }
}
