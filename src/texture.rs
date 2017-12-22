use gl;
use gl::types::*;

pub struct Texture {
    pub gl_id: GLuint,
}

#[derive(Debug)]
pub struct TextureError {
    pub message: String,
}

pub enum TextureFormat {
    R,
    Rgb,
    Rgba,
}

pub enum TextureFilter {
    Nearest,
    Linear,
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
        texture.set_filters(TextureFilter::Nearest, TextureFilter::Nearest);
        Ok(texture)
    }

    pub fn set_filters(&mut self, min: TextureFilter, mag: TextureFilter) {
        self.bind(0);
        unsafe {
            let gl_min = match min {
                TextureFilter::Nearest => gl::NEAREST,
                TextureFilter::Linear => gl::LINEAR,
            };
            let gl_mag = match mag {
                TextureFilter::Nearest => gl::NEAREST,
                TextureFilter::Linear => gl::LINEAR,
            };
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl_min as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl_mag as _);
        }
    }

    pub fn bind(&mut self, index: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + index);
            gl::BindTexture(gl::TEXTURE_2D, self.gl_id);
        }
    }

    pub fn upload_image_2d(&mut self, format: TextureFormat, width: usize, height: usize, pixels: &[u8]) {
        let gl_format = match format {
            TextureFormat::R => gl::RED,
            TextureFormat::Rgb => gl::RGB,
            TextureFormat::Rgba => gl::RGBA,
        };
        self.bind(0);
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl_format as _,
                width as _,
                height as _,
                0,
                gl_format,
                gl::UNSIGNED_BYTE,
                pixels.as_ptr() as *const _
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
