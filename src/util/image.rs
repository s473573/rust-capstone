use crate::error::CliError;

use image::io::Reader as ImageReader;
use image::RgbaImage;
use std::fs;
use std::io::Cursor;
use std::path::Path;

pub fn load_image<P: AsRef<Path>>(path: P) -> Result<Image, CliError> {
    // the image::open() method would open the file by format and fail to load fake-jpgs for leastbit
    let raw_img = fs::read(path)?;

    let reader = ImageReader::new(Cursor::new(raw_img))
        .with_guessed_format()?
        .decode()?;
    let img = reader.into_rgba8();

    let bytes: Vec<u8> = img.clone().into_vec();
    Ok(Image {
        buffer: bytes,
        dimensions: img.dimensions(),
    })
}

pub fn write_image<P: AsRef<Path>>(img: Image, path: P) -> Result<(), CliError> {
    // actually, no matter the filetype 'path' has, the image's needs to be saved as png in least-bit case. jpeg loses data.
    // but i wonder whether are there more lossless formats,
    // converting all of them would to png would be cruel.
    // if only i could check for the type being lossy and only then using png as a fallback.
    let Image {
        buffer: bytes,
        dimensions: (width, height),
    } = img;

    RgbaImage::from_vec(width, height, bytes)
        .unwrap_or_default() // would erroring here generate a black image? true defeat...
        .save_with_format(path, image::ImageFormat::Png)
        .map_err(CliError::Image)?;
    Ok(())
}

// a quick type to contain everything required for the job
pub struct Image {
    pub buffer: Vec<u8>,
    pub dimensions: (u32, u32),
}
impl Image {
    pub fn new(data: Vec<u8>, width: u32, height: u32) -> Self {
        Image {
            buffer: data,
            dimensions: (width, height),
        }
    }
}
