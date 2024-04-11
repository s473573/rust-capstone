fn main() -> Result<(), Box<dyn std::error::Error>>
{ unimplemented!() }

#[cfg(test)]
mod spec {
    use anyhow::Result;
    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use bitvec::vec::BitVec;
    use image::DynamicImage;
    
    use image::{codecs::jpeg::JpegEncoder, RgbImage};
    use pixeldust_wip::{LeastBit, Steganography};
    use std::io::Cursor;

    fn create_black_jpeg(width: u32, height: u32) -> Vec<u8> {
        //creating an initial buffer
        let mut img = RgbImage::new(width, height);

        // filling our black image
        for y in 0..height {
          for x in 0..width {
            img.put_pixel(x, y, image::Rgb([0, 0, 0]));
          }
        }
        
        let img = DynamicImage::ImageRgb8(img);

        let mut writer = Cursor::new(Vec::new());

        // encoding as jpeg with a high quality...
        let mut encoder = JpegEncoder::new_with_quality(&mut writer, 95);
        encoder.encode(
          img.as_bytes(),
          width,
          height,
          image::ColorType::Rgb8).expect("Failed to encode.");

        writer.into_inner()
    }

    #[test]
    fn embed_lb() -> Result<()> {
        // take an image
        // edit the image
        // it's not the same
        
        let steg = LeastBit {};

        let mut experiment: Vec<u8> = create_black_jpeg(25, 25);
        let before = experiment.clone();

        steg.embed(&mut experiment, b"secret!")?;
        
        assert_ne!(before, experiment);
        Ok(())
    }
    #[test]
    fn extract_lb() -> Result<()> {
        let steg = LeastBit {};

        let mut experiment: Vec<u8> = create_black_jpeg(50, 50);
        steg.embed(&mut experiment, b"secret!")?;
        let secret = steg.extract(&experiment)?;
        
        assert_eq!(String::from_utf8(secret).unwrap(), "secret!");
        Ok(())
    }
}