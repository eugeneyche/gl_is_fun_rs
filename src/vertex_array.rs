use gl;
use gl::types::*;

use buffer::{BoundVertexBuffer, BoundIndexBuffer};

pub enum VertexAttribTy {
    Float,
    FloatVec2,
    FloatVec3,
    FloatVec4,
}

pub struct VertexAttrib {
    pub ty: VertexAttribTy,
    pub offset: usize,
    pub normalized: bool,
}

pub enum DrawTy {
    Points,
    Lines,
    Triangles,
    TriangleStrip,
}

impl DrawTy {
    pub fn to_gl_draw_ty(&self) -> GLenum {
        match *self {
            DrawTy::Points => gl::POINTS,
            DrawTy::Lines => gl::LINES,
            DrawTy::Triangles => gl::TRIANGLES,
            DrawTy::TriangleStrip => gl::TRIANGLE_STRIP,
        }
    }
}

pub struct VertexArray {
    pub gl_id: GLuint,
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

    pub fn bind<'a>(&'a mut self, vertex_buffer: BoundVertexBuffer<'a>) -> BoundVertexArray<'a> {
        BoundVertexArray::new(self, vertex_buffer)
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

pub struct BoundVertexArray<'a> {
    vertex_array: &'a mut VertexArray,
    vertex_buffer: BoundVertexBuffer<'a>,
}

impl<'a> BoundVertexArray<'a> {
    pub fn new(vertex_array: &'a mut VertexArray, vertex_buffer: BoundVertexBuffer<'a>) -> Self {
        unsafe {
            gl::BindVertexArray(vertex_array.gl_id);
        }
        BoundVertexArray {
            vertex_array: vertex_array,
            vertex_buffer: vertex_buffer,
        }
    }

    pub fn set_vertex_attribs(&mut self, stride: usize, attribs: &[VertexAttrib]) {
        for (i, attrib) in attribs.iter().enumerate() {
            let (size, gl_ty) = match attrib.ty {
                VertexAttribTy::Float => (1, gl::FLOAT),
                VertexAttribTy::FloatVec2 => (2, gl::FLOAT),
                VertexAttribTy::FloatVec3 => (3, gl::FLOAT),
                VertexAttribTy::FloatVec4 => (4, gl::FLOAT),
            };
            unsafe {
                gl::EnableVertexAttribArray(i as _);
                gl::VertexAttribPointer(
                    i as _,
                    size,
                    gl_ty,
                    attrib.normalized as _,
                    stride as _,
                    attrib.offset as *const _,
                );
            }
        }
    }

    pub fn draw_arrays(&mut self, draw_ty: DrawTy, first: usize, count: usize) {
        unsafe {
            gl::DrawArrays(draw_ty.to_gl_draw_ty(), first as _, count as _);
        }
    }

    pub fn draw_elements<'b>(
        &mut self,
        draw_ty: DrawTy,
        index_buffer: BoundIndexBuffer<'b>,
        count: usize
    ) {
        unsafe {
            gl::DrawElements(
                draw_ty.to_gl_draw_ty(),
                count as _,
                gl::UNSIGNED_INT,
                0 as *const _,
            );
        }
    }
}

#[derive(Debug)]
pub struct VertexArrayError {
    pub message: String,
}

