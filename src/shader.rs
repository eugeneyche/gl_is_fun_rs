use gl;
use gl::types::*;
use std;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub enum ShaderTy {
    Fragment,
    Geometry,
    Vertex,
}

#[derive(Debug)]
pub struct ShaderError {
    pub message: String,
    pub info_log: Option<String>,
}

pub struct Shader {
    pub ty: ShaderTy,
    pub gl_id: GLuint,
}

impl ShaderTy {
    pub fn to_gl_shader_ty(&self) -> GLenum {
        match *self {
            ShaderTy::Fragment => gl::FRAGMENT_SHADER,
            ShaderTy::Geometry => gl::GEOMETRY_SHADER,
            ShaderTy::Vertex => gl::VERTEX_SHADER,
        }
    }
}

impl Shader {
    pub fn from_file(ty: ShaderTy, path: &Path) -> Result<Self, ShaderError> {
        let mut file = File::open(path).map_err(|_| {
            ShaderError {
                message: format!("Failed to open file {:?}", path),
                info_log: None,
            }
        })?;
        let mut source = String::new();
        file.read_to_string(&mut source).map_err(|_| {
            ShaderError {
                message: format!("Failed to read file {:?}", path),
                info_log: None,
            }
        })?;
        Shader::from_source(ty, &source)
    }

    pub fn from_source(ty: ShaderTy, source: &str) -> Result<Self, ShaderError> {
        let gl_id = unsafe {
            let gl_id = gl::CreateShader(ty.to_gl_shader_ty());
            if gl_id == 0 {
                return Err(ShaderError {
                    message: "Failed to create GPU shader".into(),
                    info_log: None,
                });
            }
            gl::ShaderSource(gl_id, 1, &(source.as_ptr() as *const _), &(source.len() as GLint));
            gl::CompileShader(gl_id);
            let mut is_compiled = 0;
            gl::GetShaderiv(gl_id, gl::COMPILE_STATUS, &mut is_compiled);
            if is_compiled == 0 {
                let mut log_len = 0;
                gl::GetShaderiv(gl_id, gl::INFO_LOG_LENGTH, &mut log_len);
                let mut log_buf = vec![0; log_len as usize];
                gl::GetShaderInfoLog(
                    gl_id,
                    log_len,
                    std::ptr::null_mut(),
                    log_buf.as_mut_ptr() as *mut _,
                );
                return Err(ShaderError {
                    message: "Failed to compile shader".into(),
                    info_log: String::from_utf8(log_buf).ok(),
                });
            }
            gl_id
        };
        Ok(Shader {
            ty: ty,
            gl_id: gl_id,
        })
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.gl_id);
        }
        self.gl_id = 0;
    }
}
