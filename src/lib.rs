mod error;
use error::CliError;

use bitvec::prelude::*;

pub trait Steganography {
    fn embed(&self, image_data: &mut BitVec<u8>, secret_data: &[u8]) -> Result<(), CliError>;
    fn extract(&self, image_data: &BitVec<u8>, secret_length: usize) -> Result<Vec<u8>, CliError>; 
}

pub struct LeastBit {}
impl Steganography for LeastBit {
    fn embed(&self, image_data: &mut BitVec<u8>, secret_data: &[u8]) -> Result<(), CliError> {
        let secret_bits: BitVec<u8> = BitVec::from_slice(secret_data);
        // testing the ability to store!
        if image_data.len() / 8 < secret_bits.len() {
            return Err(CliError::Image("Provided image is too small to store your data.".to_string()));
        }
        
        for (i, bit) in secret_bits.iter().enumerate() {
            let bit_i = i * 8;
            image_data.set(bit_i, *bit);
        }
        Ok(())
    }
    fn extract(&self, image_data: &BitVec<u8>, secret_length: usize) -> Result<Vec<u8>, CliError> {
        let mut extracted_bits: BitVec<u8> = BitVec::with_capacity(secret_length*8);
        for i in 0..secret_length*8 {
            let bit_i = i * 8;
            extracted_bits.push(image_data[bit_i]);
        }
        Ok(extracted_bits.into_vec())
    }
}