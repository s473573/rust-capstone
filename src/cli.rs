use clap::{Parser, Subcommand};
use stool::{LeastBit, Steganography};
use tracing::{info, error};
use std::{fs::File, io::Write, path::PathBuf};

use crate::image_utils;

#[derive(Parser, Debug)]
#[command(author, version, about = "Stool - A Stenography tool for hiding data within files", long_about = None)]
struct Args {
    #[command(subcommand)]
    mode: Mode,

    #[arg(short = 'm', long, default_value = "least_bit")]
    method: SteganographyMethod,

    #[arg(short, long)]
    key: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Mode {
    /// Makes a secret
    Insert {
        /// The input file to use
        input_file: PathBuf,

        /// The output file where the secret will be embedded
        output_file: PathBuf,

        /// The secret message to embed
        message: String,
    },
    /// Extracts a secret from a file, not always a meaningful one
    Extract {
        /// The input file containing a secret
        input_file: PathBuf,        
    },
}

#[derive(Debug, Clone, PartialEq)]
enum SteganographyMethod {
    DCTC,
    LeastBit,
    AmendZip,
}

impl std::str::FromStr for SteganographyMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dctc" => Ok(SteganographyMethod::DCTC),
            "least_bit" => Ok(SteganographyMethod::LeastBit),
            "amend_zip" => Ok(SteganographyMethod::AmendZip),
            _ => Err(format!("Invalid steganography method: {}", s)),
        }
    }
}

pub fn main() -> Result<(), stool::error::CliError> {
    let args = Args::parse();
    match args.mode {
        Mode::Insert { input_file, output_file, message } => {
            info!("Inserting message '{}' into '{}' and saving as '{}' (method: {:?}, key: {:?})",
                     message, input_file.display(), output_file.display(), args.method, args.key);

            let mut image= image_utils::load_image(input_file)?;
            let steg = LeastBit{};
            steg.embed(&mut image.buffer, message.as_bytes())?;

            image_utils::write_image(image, &output_file)
        }
        Mode::Extract { input_file } => {
            info!("Extracting secret from '{}' (method: {:?}, key: {:?})", 
                    input_file.display(), args.method, args.key);
            
            let image = image_utils::load_image(input_file)?;
            let steg = LeastBit{};
            let secret = steg.extract(&image.buffer)?;
            // only print when the secret is text in form
            // println!("{}", String::from_utf8(secret.clone()).unwrap());
            let secret_path= PathBuf::from("secret");

            std::fs::write(secret_path, secret)?;
            info!("Secret recovered successfully.");
            Ok(())
        }
    }
}
