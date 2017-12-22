use gl;
use gl::types::*;
use std;

pub struct Buffer {
    pub gl_id: GLuint,
}

pub enum BufferUsageHint {
    DynamicRead,
    DynamicDraw,
    StaticRead,
    StaticDraw,
    StreamRead,
    StreamDraw,
}

#[derive(Debug)]
pub struct BufferError {
    pub message: String,
}

impl Buffer {
    pub fn new() -> Result<Buffer, BufferError> {
        let gl_id = unsafe {
            let mut gl_id = 0;
            gl::GenBuffers(1, &mut gl_id);
            if gl_id == 0 {
                return Err(BufferError {
                    message: "Failed to create GPU buffer".into(),
                });
            }
            gl_id
        };
        Ok(Buffer {
            gl_id: gl_id,
        })
    }

    fn upload_to_target<T: Sized>(&mut self, target: GLenum, data: &[T], usage_hint: BufferUsageHint) {
        let gl_usage_hint = match usage_hint {
            BufferUsageHint::DynamicRead => gl::DYNAMIC_READ,
            BufferUsageHint::DynamicDraw => gl::DYNAMIC_DRAW,
            BufferUsageHint::StaticRead => gl::DYNAMIC_READ,
            BufferUsageHint::StaticDraw => gl::DYNAMIC_DRAW,
            BufferUsageHint::StreamRead => gl::STREAM_READ,
            BufferUsageHint::StreamDraw => gl::STREAM_DRAW,
        };
        unsafe {
            gl::BindBuffer(target, self.gl_id);
            gl::BufferData(
                target,
                (std::mem::size_of::<T>() * data.len()) as _,
                data.as_ptr() as *const _,
                gl_usage_hint,
            );
        }
    }

    pub fn upload_array<T: Sized>(&mut self, data: &[T], usage_hint: BufferUsageHint) {
        self.upload_to_target(gl::ARRAY_BUFFER, data, usage_hint);
    }

    pub fn upload_element_array<T: Sized>(&mut self, data: &[T], usage_hint: BufferUsageHint) {
        self.upload_to_target(gl::ELEMENT_ARRAY_BUFFER, data, usage_hint);
    }

    pub fn bind(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.gl_id);
        }
    }

    pub fn bind_element(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.gl_id);
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.gl_id);
        }
        self.gl_id = 0;
    }
}
