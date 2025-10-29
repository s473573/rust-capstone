use stool::steg::Steganography;
use stool::steg::{lsb::LeastBit, SteganographyMethod};
use stool::util::crypt as crypt_utils;
use stool::util::image as image_utils;

use tracing::info;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Stool - A Stenography tool for hiding data within files", long_about = None)]
struct Args {
    #[command(subcommand)]
    mode: Mode,

    #[arg(short = 'm', long, default_value = "least_bit")]
    method: SteganographyMethod,

    /// Apply a password to secure your secret with symmetrical encryption
    #[arg(short, long)]
    password: Option<String>,
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

        /// Optional file to write recovered secret bytes to.
        /// If omitted, the secret is printed to stdout.
        #[arg(short = 'o', long = "out")]
        output_file: Option<PathBuf>,
    },
}

pub fn main() -> Result<(), stool::error::CliError> {
    let args = Args::parse();
    match args.mode {
        Mode::Insert { input_file, output_file, message } => {
            info!("Inserting secret into '{}' and saving as '{}' (method: {:?})",
                     input_file.display(), output_file.display(), args.method);
            
            let mut image= image_utils::load_image(input_file)?;
            
            let message_bytes = args.password
                .as_ref()
                .map(|p| crypt_utils::encrypt_message(p, &message))
                .transpose()?
                .unwrap_or_else(|| message.as_bytes().to_vec());
            
            let steg = LeastBit{};
            steg.embed(&mut image.buffer, &message_bytes)?;

            image_utils::write_image(image, &output_file)
        }
        Mode::Extract { input_file, output_file } => {
            info!("Extracting secret from '{}' (method: {:?}, key: {:?})", 
                    input_file.display(), args.method, args.password);
            
            let image = image_utils::load_image(input_file)?;
            let steg = LeastBit{};

            let buffer = steg.extract(&image.buffer)?;

            let secret = args.password
                .as_ref()
                .map(|p| crypt_utils::decrypt_message(p, &buffer))
                .transpose()?
                .unwrap_or(buffer);

            if let Some(path) = output_file {
                std::fs::write(&path, &secret)?;
                info!("Secret recovered and written to {}", path.display());
            } else {
                // first try to show in human-readable form
                if let Ok(as_utf8) = String::from_utf8(secret.clone()) {
                    println!("{as_utf8}");
                } else {
                    // fallback: output base64 code
                    use base64::{engine::general_purpose, Engine as _};
                    let b64 = general_purpose::STANDARD.encode(&secret);
                    println!("{b64}");
                }
                info!("Secret recovered successfully, printed to stdout.");
            }

            Ok(())
        }
    }
}
