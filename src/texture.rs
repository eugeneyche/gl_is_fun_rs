use gl;
use gl::types::*;

pub enum TextureFormat {
    R,
    Rgb,
    Rgba,
}

pub enum TextureFilter {
    Linear,
    Nearest,
}

#[derive(Debug)]
pub struct TextureError {
    pub message: String,
}

pub struct Texture {
    pub gl_id: GLuint,
}

pub struct BoundTexture<'a> {
    pub unit_index: u32,
    texture: &'a mut Texture,
}

impl TextureFormat {
    pub fn to_gl_format(&self) -> GLenum {
        match *self {
            TextureFormat::R => gl::RED,
            TextureFormat::Rgb => gl::RGB,
            TextureFormat::Rgba => gl::RGBA,
        }
    }
}

impl TextureFilter {
    pub fn to_gl_filter(&self) -> GLenum {
        match *self {
            TextureFilter::Linear => gl::LINEAR,
            TextureFilter::Nearest => gl::NEAREST,
        }
    }
}

impl Texture {
    pub fn new() -> Result<Self, TextureError> {
        let gl_id = unsafe {
            let mut gl_id = 0;
            gl::GenTextures(1, &mut gl_id);
            if gl_id == 0 {
                return Err(TextureError {
                    message: "Failed to create GPU texture".into(),
                });
            }
            gl_id
        };
        let mut texture = Texture { gl_id: gl_id };
        texture.bind(0).set_filters(TextureFilter::Nearest, TextureFilter::Nearest);
        Ok(texture)
    }

    pub fn bind<'a>(&'a mut self, unit_index: u32) -> BoundTexture<'a> {
        BoundTexture::new(unit_index, self)
    }
}

impl<'a> BoundTexture<'a> {
    pub fn new(unit_index: u32, texture: &'a mut Texture) -> Self {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit_index);
            gl::BindTexture(gl::TEXTURE_2D, texture.gl_id);
        }
        BoundTexture {
            unit_index: unit_index,
            texture: texture,
        }
    }

    pub fn set_filters(&mut self, min: TextureFilter, mag: TextureFilter) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + self.unit_index);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                min.to_gl_filter() as _,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                mag.to_gl_filter() as _,
            );
        }
    }

    pub fn upload_image_2d(&mut self, format: TextureFormat, width: usize, height: usize, pixels: &[u8]) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + self.unit_index);
            let gl_format = format.to_gl_format();
            gl::ActiveTexture(gl::TEXTURE0 + self.unit_index);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl_format as _,
                width as _,
                height as _,
                0,
                gl_format,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as *const _,
            );
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.gl_id);
        }
        self.gl_id = 0;
    }
}
