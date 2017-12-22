use gl;
use gl::types::*;

pub struct VertexArray {
    pub gl_id: GLuint,
}

#[derive(Debug)]
pub struct VertexArrayError {
    pub message: String,
}

impl VertexArray {
    pub fn new() -> Result<VertexArray, VertexArrayError> {
        let gl_id = unsafe {
            let mut gl_id = 0;
            gl::GenVertexArrays(1, &mut gl_id);
            if gl_id == 0 {
                return Err(VertexArrayError {
                    message: "Failed to create GPU vertex array".into(),
                });
            }
            gl_id
        };

        Ok(VertexArray {
            gl_id: gl_id,
        })
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindVertexArray(self.gl_id);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.gl_id);
        }
        self.gl_id = 0;
    }
}
