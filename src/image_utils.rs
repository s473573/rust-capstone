use std::io::{Cursor, Read};
use std::{io::Write, path::Path};
use std::fs::{self, File};
use image::{GenericImageView, RgbaImage};
use image::io::Reader as ImageReader;

use crate::error::CliError;

pub fn load_image<P: AsRef<Path>>(path: P) -> Result<Image, CliError> {
    // the image::open() method would open the file by format and fail to load fake-jpgs for leastbit
    let raw_img = fs::read(path)?;
    
    let reader = ImageReader::new(Cursor::new(raw_img))
        .with_guessed_format()?
        .decode()?;
    let img = reader.into_rgba8();

    let bytes: Vec<u8> = img.clone().into_vec();
    Ok(Image{buffer: bytes, dimensions: img.dimensions()})
}

pub fn write_image<P: AsRef<Path>>(img: Image, path: P) -> Result<(), CliError> {
    // actually, no matter the filetype 'path' has, the image's needs to be saved as png in least-bit case. jpeg loses data.
    // but i wonder whether are there more lossless formats,
    // converting all of them would to png would be cruel.
    // if only i could check for the type being lossy and only then using png as a fallback. 
    let Image {
        buffer: bytes,
        dimensions: (width, height)
    } = img;

    RgbaImage::from_vec(width, height, bytes).unwrap_or_default() // erroring would generate a black image , indicating defeat...
        .save_with_format(path, image::ImageFormat::Png)
        .map_err(|err| CliError::Image("Fatal error while writing the image".to_owned())) // later do something useful with it
}

// a quick type to contain everything required for the job
pub struct Image {
    pub buffer: Vec<u8>,
    pub dimensions: (u32, u32)
}
impl Image{
    pub fn new(data: Vec<u8>, width: u32, height: u32) -> Self {
        Image{ buffer: data, dimensions: (width, height) }
    }
}