extern crate gl;
extern crate glutin;
extern crate image as piston_image;

mod buffer;
mod geometry;
mod image;
mod shader;
mod shader_program;
mod texture;
mod vertex_array;

use glutin::GlContext;
use std::path::Path;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("GL Fun!")
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
        &shader::Shader::from_file(shader::ShaderTy::Vertex, Path::new("res/shaders/basic.vert")).unwrap(),
        &shader::Shader::from_file(shader::ShaderTy::Fragment, Path::new("res/shaders/basic.frag")).unwrap(),
    ]).unwrap();
    let tex_loc = program.get_uniform_location("tex").unwrap_or(-1 as _);
    let img = image::Image::from_file(Path::new("res/images/squid.png")).unwrap();
    let tex_format = match img.format {
        image::ImageFormat::R => texture::TextureFormat::R,
        image::ImageFormat::Rgb => texture::TextureFormat::Rgb,
        image::ImageFormat::Rgba => texture::TextureFormat::Rgba,
        _ => panic!("Unsupported image format"),
    };
    let mut tex = texture::Texture::new().unwrap();
    tex.upload_image_2d(
        tex_format,
        img.width,
        img.height,
        &img.pixels,
    );
    tex.bind(0);
    let mut triangle = geometry::Geometry::new().unwrap();
    triangle.set_vertices(
        &[
            [-1.0f32, 1.0, 0.0, 1.0],
            [-1.0f32,-1.0, 0.0, 0.0],
            [ 1.0f32, 1.0, 1.0, 1.0],
            [ 1.0f32,-1.0, 1.0, 0.0],
        ],
        &[
            geometry::VertexAttrib {
                ty: geometry::VertexAttribTy::FloatVec2,
                offset: 0,
                normalized: false,
            },
            geometry::VertexAttrib {
                ty: geometry::VertexAttribTy::FloatVec2,
                offset: 4 * 2,
                normalized: false,
            },
        ],
    );
    while running {
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
        program.activate();
        unsafe {
            gl::Uniform1i(tex_loc as _, 0);
        }
        triangle.draw(geometry::DrawTy::TriangleStrip);
        gl_window.swap_buffers();
    }
}
