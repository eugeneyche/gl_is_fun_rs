use piston_image;
use std::path::Path;

pub struct Image {
    pub format: ImageFormat,
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

pub enum ImageFormat {
    R,
    Ra,
    Rgb,
    Rgba,
}

#[derive(Debug)]
pub struct ImageError {
    pub message: String
}

impl Image {
    pub fn from_file(path: &Path) -> Result<Self, ImageError> {
        let dynamic_img = piston_image::open(path).map_err(|_| {
            ImageError {
                message: format!("Failed to read image {:?}", path),
            }
        })?.flipv();
        let (format, width, height, pixels) = match dynamic_img {
            piston_image::DynamicImage::ImageLuma8(img) => {
                (ImageFormat::R, img.width(), img.height(), img.into_raw())
            },
            piston_image::DynamicImage::ImageLumaA8(img) => {
                (ImageFormat::Ra, img.width(), img.height(), img.into_raw())
            },
            piston_image::DynamicImage::ImageRgb8(img) => {
                (ImageFormat::Rgb, img.width(), img.height(), img.into_raw())
            },
            piston_image::DynamicImage::ImageRgba8(img) => {
                (ImageFormat::Rgba, img.width(), img.height(), img.into_raw())
            },
        };
        Ok(Image {
            format: format,
            width: width as _,
            height: height as _,
            pixels: pixels,
        })
    }
}
