mod error;
use error::CliError;

trait Steganography {
    fn embed(&self, image_data: &mut Vec<u8>, secret_data: &[u8]) -> Result<(), CliError>;
    fn extract(&self, image_data: &[u8]) -> Result<Vec<u8>, CliError>; 
}

struct LeastBit {}