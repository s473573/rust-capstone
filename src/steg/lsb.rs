use super::CliError;
use super::Steganography;

use bitvec::prelude::*;

// disadvantage of the least bit: relatively small secrets. works with text but what about other media?
const HEADER_SIZE: usize = 8; // 8 bits of header, all of it is currently used for indicating payload size
                              // stores the payload size in BYTES

pub struct LeastBit;
impl LeastBit {
    fn read_lsb(src: &[u8]) -> BitVec<u8, Msb0> {
        let mut bits: BitVec<u8, Msb0> = BitVec::with_capacity(src.len() * 8);
        for &b in src {
            let lsb = b & 1;
            bits.push(lsb != 0);
        }
        bits
    }

    fn write_lsb(target: &mut Vec<u8>, secret_bits: BitVec<u8>) {
        let mut target_bits: BitVec<u8> = BitVec::from_vec(target.clone()); // least bit first bitvec
        for (i, bit) in secret_bits.iter().enumerate() {
            let bit_i = i * 8;
            target_bits.set(bit_i, *bit)
        }

        *target = target_bits.into_vec()
    }
}
/// Implementation of the `Steganography` trait for the Least Bit steganography format.
///
/// Consists of methods for reading the least significant bits of image-pixels and writing to them from a byte-vector.
impl Steganography for LeastBit {
    fn embed(&self, source_data: &mut Vec<u8>, secret_data: &[u8]) -> Result<(), CliError> {
        let embed_len = secret_data.len() as u8; // LEN is written in BYTES

        let mut secret_bits = BitVec::<u8>::new();

        // header construction
        let header = BitVec::<u8, Msb0>::from_element(embed_len);
        assert_eq!(header.len(), HEADER_SIZE);

        let payload_bits: BitVec<u8, Msb0> = BitVec::from_slice(secret_data);

        secret_bits.extend(header);
        secret_bits.extend(payload_bits);

        // testing the ability to store!
        if source_data.len() / 8 < secret_bits.len() {
            return Err(CliError::Misc(
                "Provided image is too small to store your data".to_string(),
            ));
        }

        LeastBit::write_lsb(source_data, secret_bits);
        Ok(())
    }
    fn extract(&self, source_data: &[u8]) -> Result<Vec<u8>, CliError> {
        let secret_bits = LeastBit::read_lsb(source_data);

        // written as a u8, read instead as a usize for comfort
        let payload_len: usize = secret_bits[0..HEADER_SIZE].load();
        assert!(payload_len > 0);
        let payload = &secret_bits[HEADER_SIZE..HEADER_SIZE + payload_len * 8];

        Ok(payload.to_owned().into_vec())
    }
}
