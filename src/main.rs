#![allow(deprecated)]
use anyhow::Result;
use tracing::error;
use tracing_subscriber::fmt;
use tracing_subscriber::EnvFilter;

mod cli;

fn main() -> Result<()> {
    setup_logging();

    if let Err(err) = cli::main() {
        error!("{err}");
        // nonzero situation
        std::process::exit(1);
    }

    Ok(())
}

fn setup_logging() {
    let sub = fmt::Subscriber::builder()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::from("info")),
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

    fn create_green_noise_png(width: u32, height: u32) -> Vec<u8> {
        //creates a new image buffer filled with random green values.

        let img = ImageBuffer::from_fn(width, height, |_, _| {
            let mut rng = rand::thread_rng();
            let green = rng.gen_range(0..=255);
            image::Rgb([0, green, 0])
        });

        let img = DynamicImage::ImageRgb8(img);
        let mut bytes: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
            .expect("No image for you!");
        bytes
    }

    #[test]
    fn embed_lb() -> Result<()> {
        // take an image
        // edit the image
        // it's not the same

        let steg = LeastBit {};

        let mut experiment: Vec<u8> = create_green_noise_png(25, 25);
        let before = experiment.clone();

        steg.embed(&mut experiment, b"secret!")?;

        assert_ne!(before, experiment);
        Ok(())
    }
    #[test]
    fn extract_lb() -> Result<()> {
        let steg = LeastBit {};

        let mut experiment: Vec<u8> = create_green_noise_png(50, 50);
        steg.embed(&mut experiment, b"secret!")?;
        let secret = steg.extract(&experiment)?;

        assert_eq!(String::from_utf8(secret).unwrap(), "secret!");
        Ok(())
    }

    #[test]
    fn test_crypto() -> Result<(), Box<dyn std::error::Error>> {
        let steg = LeastBit {};

        let mut experiment: Vec<u8> = create_green_noise_png(50, 50);

        let secret = encrypt_message("pass", "secret!")?;
        steg.embed(&mut experiment, &secret)?;
        let secret = steg.extract(&experiment)?;
        let secret = decrypt_message("pass", &secret)?;

        assert_eq!(String::from_utf8(secret).unwrap(), "secret!");
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_barebones() -> Result<(), Box<dyn std::error::Error>> {
        let tmp_dir = assert_fs::TempDir::new().unwrap();

        let image_file = tmp_dir.child("simulacrum.png");
        image_file.touch()?;
        image_file.write_binary(&create_green_noise_png(640, 480))?; // simulating some image data

        let stego_path = tmp_dir.child("message.png");
        let mut cmd = Command::cargo_bin("stool")?;
        cmd.arg("insert")
            .arg(image_file.path())
            .arg(stego_path.path())
            .arg("secret!");
        cmd.assert().success();

        // it suffices to check for overwritten file's existence
        stego_path.assert(predicates::path::exists());

        // run `extract` into a file in the SAME temp dir
        let recovered_path = tmp_dir.child("recovered.bin");
        let mut cmd = Command::cargo_bin("stool")?;
        cmd.arg("extract")
            .arg(stego_path.path())
            .arg("--out")
            .arg(recovered_path.path());
        cmd.assert().success();

        // read recovered secret
        recovered_path.assert(predicates::path::exists());
        let recovered = std::fs::read(recovered_path.path())?;
        assert_eq!(recovered, b"secret!");

        println!("Filesystem operations work!");

        Ok(())
    }
}
