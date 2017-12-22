use gl;
use std;

/*

use buffer::{Buffer, BufferUsageHint};
use vertex_array::VertexArray;

pub struct Geometry {
    vertex_array: VertexArray,
    vertex_buffer: Buffer,
    index_buffer: Option<Buffer>,
    num_vertices: usize,
}

#[derive(Debug)]
pub struct GeometryError {
    pub message: String,
}

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

impl Geometry {
    pub fn new() -> Result<Self, GeometryError> {
        let vertex_array = VertexArray::new().map_err(|err| {
            GeometryError {
                message: err.message,
            }
        })?;
        let vertex_buffer = Buffer::new().map_err(|err| {
            GeometryError {
                message: err.message,
            }
        })?;
        Ok(Geometry {
            vertex_array: vertex_array,
            vertex_buffer: vertex_buffer,
            index_buffer: None,
            num_vertices: 0,
        })
    }

    fn set_vertex_attribs(&mut self, attribs: &[VertexAttrib], stride: usize) {
        self.vertex_array.bind();
        self.vertex_buffer.bind();
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

    pub fn set_vertices<T: Sized>(&mut self, vertices: &[T], attribs: &[VertexAttrib]) {
        self.set_vertex_attribs(attribs, std::mem::size_of::<T>());
        self.vertex_buffer.upload_array(vertices, BufferUsageHint::DynamicDraw);
        self.num_vertices = vertices.len();
        self.index_buffer = None;
    }

    pub fn set_vertices_with_indices<T: Sized>(&mut self, vertices: &[T], attribs: &[VertexAttrib], indices: &[u32]) {
        self.set_vertices(vertices, attribs);
        let mut index_buffer = Buffer::new().unwrap();
        index_buffer.upload_element_array(indices, BufferUsageHint::DynamicRead);
        self.index_buffer = Some(index_buffer);
        self.num_vertices = indices.len();
    }

    pub fn draw(&mut self, draw_ty: DrawTy) {
        let gl_mode = match draw_ty {
            DrawTy::Points => gl::POINTS,
            DrawTy::Lines => gl::LINES,
            DrawTy::Triangles => gl::TRIANGLES,
            DrawTy::TriangleStrip => gl::TRIANGLE_STRIP,
        };
        self.vertex_array.bind();
        if let Some(ref mut index_buffer) = self.index_buffer {
            index_buffer.bind_element();
            unsafe {
                gl::DrawElements(gl_mode, self.num_vertices as _, gl::UNSIGNED_INT, 0 as *const _);
            }
        } else {
            unsafe {
                gl::DrawArrays(gl_mode, 0, self.num_vertices as _);
            }
        }
    }

    pub fn draw_instanced(&mut self, draw_ty: DrawTy, num_instances: u32) {
        let gl_mode = match draw_ty {
            DrawTy::Points => gl::POINTS,
            DrawTy::Lines => gl::LINES,
            DrawTy::Triangles => gl::TRIANGLES,
            DrawTy::TriangleStrip => gl::TRIANGLE_STRIP,
        };
        self.vertex_array.bind();
        if let Some(ref mut index_buffer) = self.index_buffer {
            index_buffer.bind_element();
            unsafe {
                gl::DrawElementsInstanced(
                    gl_mode,
                    self.num_vertices as _,
                    gl::UNSIGNED_INT,
                    0 as *const _,
                    num_instances as _,
                );
            }
        } else {
            unsafe {
                gl::DrawArraysInstanced(gl_mode, 0, self.num_vertices as _, num_instances as _);
            }
        }
    }
}
*/
