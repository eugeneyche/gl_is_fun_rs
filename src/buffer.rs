use gl;
use gl::types::*;
use std;
use std::marker::PhantomData;

pub trait BufferTy {
    fn to_gl_buffer_ty() -> GLenum;
}

pub struct IndexBufferTy;
pub struct VertexBufferTy;

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

pub struct Buffer<Ty: BufferTy> {
    ty: PhantomData<Ty>,
    pub gl_id: GLuint,
}

pub struct BoundBuffer<'a, Ty: 'a + BufferTy> {
    buffer: &'a mut Buffer<Ty>,
}

pub type VertexBuffer = Buffer<VertexBufferTy>;
pub type BoundVertexBuffer<'a> = BoundBuffer<'a, VertexBufferTy>;
pub type IndexBuffer = Buffer<IndexBufferTy>;
pub type BoundIndexBuffer<'a> = BoundBuffer<'a, IndexBufferTy>;

impl BufferTy for VertexBufferTy {
    fn to_gl_buffer_ty() -> GLenum { gl::ARRAY_BUFFER }
}

impl BufferTy for IndexBufferTy {
    fn to_gl_buffer_ty() -> GLenum { gl::ELEMENT_ARRAY_BUFFER }
}

impl BufferUsageHint {
    fn to_gl_usage_hint(&self) -> GLenum {
        match *self {
            BufferUsageHint::DynamicRead => gl::DYNAMIC_READ,
            BufferUsageHint::DynamicDraw => gl::DYNAMIC_DRAW,
            BufferUsageHint::StaticRead => gl::DYNAMIC_READ,
            BufferUsageHint::StaticDraw => gl::DYNAMIC_DRAW,
            BufferUsageHint::StreamRead => gl::STREAM_READ,
            BufferUsageHint::StreamDraw => gl::STREAM_DRAW,
        }
    }
}

impl<Ty: BufferTy> Buffer<Ty> {
    pub fn new() -> Result<Self, BufferError> {
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
        Ok(Buffer::<Ty> {
            ty: PhantomData,
            gl_id: gl_id,
        })
    }

    pub fn bind<'a>(&'a mut self) -> BoundBuffer<'a, Ty> {
        BoundBuffer::new(self)
    }
}

impl<Ty: BufferTy> Drop for Buffer<Ty> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.gl_id);
        }
        self.gl_id = 0;
    }
}

impl<'a, Ty: BufferTy> BoundBuffer<'a, Ty> {
    fn new(buffer: &'a mut Buffer<Ty>) -> Self {
        unsafe {
            gl::BindBuffer(Ty::to_gl_buffer_ty(), buffer.gl_id);
        }
        BoundBuffer {
            buffer: buffer,
        }
    }

    pub fn upload<Datum: Sized>(&mut self, data: &[Datum], usage_hint: BufferUsageHint) {
        unsafe {
            gl::BufferData(
                Ty::to_gl_buffer_ty(),
                (std::mem::size_of::<Datum>() * data.len()) as _,
                data.as_ptr() as *const _,
                usage_hint.to_gl_usage_hint(),
            );
        }
    }
}
