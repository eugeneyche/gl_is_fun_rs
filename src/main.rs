extern crate cgmath;
extern crate gl;
extern crate glutin;
extern crate image as piston_image;

mod buffer;
mod image;
mod shader;
mod shader_program;
mod texture;
mod vertex_array;

use cgmath::prelude::*;
use glutin::GlContext;
use std::path::Path;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Open GL is fun!")
        .with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new()
        .with_gl(
            glutin::GlRequest::Specific(
                glutin::Api::OpenGl,
                (3, 3),
            )
        )
        .with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
    unsafe {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    }
    let mut running = true;
    let mut program = shader_program::ShaderProgram::link(&[
        &shader::Shader::from_file(
            shader::ShaderTy::Vertex,
            Path::new("res/shaders/basic.vert")
        ).unwrap(),
        &shader::Shader::from_file(
            shader::ShaderTy::Fragment,
            Path::new("res/shaders/basic.frag")
        ).unwrap(),
    ]).unwrap();
    let tex_loc = program.get_uniform_location("tex");
    let wvp_loc = program.get_uniform_location("wvp");
    let img = image::Image::from_file(Path::new("res/images/squid.png")).unwrap();
    let mut squid_tex = texture::Texture::new().unwrap();
    squid_tex
        .bind(0)
        .upload_image_2d(
            img.format.to_texture_format().unwrap(),
            img.width,
            img.height,
            &img.pixels,
        );
    let mut vertex_array = vertex_array::VertexArray::new().unwrap();
    let mut vertex_buffer = buffer::VertexBuffer::new().unwrap();
    vertex_buffer
        .bind()
        .upload(
            &[
                [-1f32,  1.3, 0., 0., 1.],
                [ 1f32,  1.3, 0., 1., 1.],
                [-1f32, -1., 0., 0., 0.],
                [ 1f32, -1., 0., 1., 0.],
            ],
            buffer::BufferUsageHint::DynamicDraw,
        );
    vertex_array
        .bind(vertex_buffer.bind())
        .set_vertex_attribs(
            5 * 4,
            &[
                vertex_array::VertexAttrib {
                    ty: vertex_array::VertexAttribTy::FloatVec3,
                    offset: 0,
                    normalized: false,
                },
                vertex_array::VertexAttrib {
                    ty: vertex_array::VertexAttribTy::FloatVec2,
                    offset: 3 * 4,
                    normalized: false,
                },
            ],
        );
    let projection = cgmath::perspective(cgmath::Rad(1f32),  1024.0 / 768.0, 0.1, 100.0);
    let view = cgmath::Matrix4::look_at(
        cgmath::Point3::new(0f32, 2.0, 10.0),
        cgmath::Point3::new(0f32, 0.0, 0.0),
        cgmath::Vector3::new(0f32, 1.0, 0.0),
    );
    let mut world = cgmath::Matrix4::<f32>::identity();
    while running {
        world = world * cgmath::Matrix4::from_angle_y(cgmath::Rad(0.05f32));
        let wvp = projection * view * world;
        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::Closed => running = false,
                    _ => ()
                },
                _ => ()
            }
        });
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
        let bound_squid_tex = squid_tex.bind(0);
        let mut active_program = program.activate();
        active_program.uniform_integer(tex_loc, bound_squid_tex.unit_index as _);
        active_program.uniform_float_mat4(wvp_loc, wvp.as_ref());
        vertex_array
            .bind(vertex_buffer.bind())
            .draw_arrays(vertex_array::DrawTy::TriangleStrip, 0, 4);
        gl_window.swap_buffers();
    }
}
