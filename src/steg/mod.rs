pub mod lsb;
pub mod other;

use crate::error::CliError;

/// The `Steganography` trait defines an interface for embedding and extracting
/// secret data within the target data.
pub trait Steganography {
    /// Embeds secret data within a mutable image data vector.
    ///
    /// # Parameters
    ///
    /// * `image_data`: A mutable reference to a vector of bytes representing pixels.
    /// * `secret_data`: A slice of bytes containing secret data to be hidden.     
    ///
    /// # Returns
    ///
    /// * `Result<(), CliError>`: Returns a quiet `Ok(())` on success, or throws a `CliError`
    ///    indicating the type of failure.
    fn embed(&self, image_data: &mut Vec<u8>, secret_data: &[u8]) -> Result<(), CliError>;

    /// Tries to extract the hidden data from an image data vector.
    ///
    /// # Parameters
    ///
    /// * `image_data`: A slice of bytes representing the image containing hidden data.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<u8>, CliError>`:  Returns a vector of bytes representing the extracted steganography
    ///     data on success, or a `CliError` indicating the type of failure.
    fn extract(&self, image_data: &[u8]) -> Result<Vec<u8>, CliError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum SteganographyMethod {
    // Embeds a secret by altering the frequency components of a JPEG image, very imperceptible
    DCTC,
    // Hides data withing the least significant bits of pixel values
    LeastBit,
    /// Annexes a file to the end of the image, barely a steganography but still useful for hiding archives
    AttachZip,
}

impl std::str::FromStr for SteganographyMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dctc" => Ok(SteganographyMethod::DCTC),
            "least_bit" => Ok(SteganographyMethod::LeastBit),
            "amend_zip" => Ok(SteganographyMethod::AttachZip),
            _ => Err(format!("Invalid steganography method: {}", s)),
        }
    }
}
