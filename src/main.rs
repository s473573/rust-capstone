use tracing_subscriber::fmt;
use tracing_subscriber::EnvFilter;
use tracing::error;

mod cli;

fn main() -> Result<(), ()>{
    setup_logging();

    cli::main().map_err(
        |err| {
            error!("{}", err);
        }
    )

}

fn setup_logging() {
    let sub = fmt::Subscriber::builder()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::from("info"))
        )
        .with_writer(std::io::stderr)
        .finish();

    tracing::subscriber::set_global_default(sub)
        .expect("Crashed while setting the global default!");
}

#[cfg(test)]
mod spec {
    use stool::steg::{lsb::*, Steganography};    
    use stool::util::crypt::{decrypt_message, encrypt_message};

    use anyhow::Result;
    use image::{DynamicImage, ImageBuffer};

    use assert_cmd::Command;
    use assert_fs::prelude::*;
    use rand::Rng;
    use std::io::Cursor;
    
    fn create_green_noise_jpeg(width: u32, height: u32) -> Vec<u8> {
        //creates a new image buffer filled with random green values.

        let img = ImageBuffer::from_fn(width, height, |_, _| {
            let mut rng = rand::thread_rng();
            let green = rng.gen_range(0..=255);
            image::Rgb([0, green, 0])
        });
    
        let img = DynamicImage::ImageRgb8(img);
        let mut bytes: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png).expect("No image for you!");
        bytes
    }

    #[test]
    fn embed_lb() -> Result<()> {
        // take an image
        // edit the image
        // it's not the same
        
        let steg = LeastBit {};

        let mut experiment: Vec<u8> = create_green_noise_jpeg(25, 25);
        let before = experiment.clone();

        steg.embed(&mut experiment, b"secret!")?;
        
        assert_ne!(before, experiment);
        Ok(())
    }
    #[test]
    fn extract_lb() -> Result<()> {
        let steg = LeastBit {};

        let mut experiment: Vec<u8> = create_green_noise_jpeg(50, 50);
        steg.embed(&mut experiment, b"secret!")?;
        let secret = steg.extract(&experiment)?;
        
        assert_eq!(String::from_utf8(secret).unwrap(), "secret!");
        Ok(())
    }
    
    #[test]
    fn test_crypto() -> Result<(), Box<dyn std::error::Error>> {
        let steg = LeastBit {};

        let mut experiment: Vec<u8> = create_green_noise_jpeg(50, 50);

        let secret = encrypt_message("pass", "secret!")?;
        steg.embed(&mut experiment, &secret)?;
        let secret = steg.extract(&experiment)?;
        let secret = decrypt_message("pass", &secret)?;
        
        assert_eq!(String::from_utf8(secret).unwrap(), "secret!");
        Ok(())
    }
    
    #[test]
    fn test_barebones() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = assert_fs::TempDir::new().unwrap();

        let image_file = tmp_dir.child("simulacrum.jpg");
        image_file.touch()?;
        image_file.write_binary(&create_green_noise_jpeg(640, 480))?; // simulating some image data

        let mut cmd = Command::cargo_bin("stool")?;
        cmd.arg("insert")
           .arg(image_file.path())
           .arg(tmp_dir.child("message.png").path())
           .arg("secret!");
        cmd.assert().success();

        // it suffices to check for overwritten file's existence
        tmp_dir.child("message.png").assert(predicates::path::exists());

        let mut cmd = Command::cargo_bin("stool")?;
        cmd.arg("extract")
           .arg(tmp_dir.child("message.png").path());
        cmd.assert().success();
        
        // move this to tempfs too
        let secret= std::fs::read("secret")
            .expect("The program failed at reading the produced image.");
        assert_eq!(secret, b"secret!");
        
        println!("Filesystem operations work!");

        Ok(())
    }
}