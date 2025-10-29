#![allow(clippy::ptr_arg)]
use crate::error::CliError;

use ring::{
    aead,
    digest::{Context, SHA256},
    rand::{SecureRandom, SystemRandom},
};
use tracing::debug;

fn construct_key(password: &str) -> aead::LessSafeKey {
    let password = password.as_bytes();
    let mut context = Context::new(&SHA256);
    context.update(password);
    let key_data = context.finish();

    let unbound_key = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, key_data.as_ref())
        .expect("Critical failure during passphrase-key conversion.");
    aead::LessSafeKey::new(unbound_key)
}

// im not really familiar with aead in cryptography, but im including it for the sake of being a best practice
fn encrypt(key: &aead::LessSafeKey, message: &str) -> Result<Vec<u8>, CliError> {
    let rand = SystemRandom::new();
    let mut nonce_bytes = [0u8; 12];
    rand.fill(&mut nonce_bytes).unwrap();

    // it seems i need to persist nonce variable for later decryption, but the crate makes it difficult
    // could it be that this current approach is incorrent?
    let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);
    let mut in_out = message.as_bytes().to_vec();
    key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut in_out)?;

    Ok([&nonce_bytes[..], &in_out].concat())
}

fn decrypt(key: &aead::LessSafeKey, message_data: &Vec<u8>) -> Result<Vec<u8>, CliError> {
    if message_data.len() < 12 {
        // can't be less the nonce size!
        return Err(ring::error::Unspecified.into());
    }

    let mut nonce = [0u8; 12];
    nonce.copy_from_slice(&message_data[0..12]);
    let nonce = aead::Nonce::assume_unique_for_key(nonce);
    debug!("got nonce");

    let mut ciphertext = message_data[12..].to_owned();
    debug!("got ciphertext");
    let plaintext = key.open_in_place(nonce, aead::Aad::empty(), &mut ciphertext)?;
    debug!("successfully decoded");

    Ok(plaintext.to_vec())
}

pub fn encrypt_message(password: &str, message: &str) -> Result<Vec<u8>, CliError> {
    let key = construct_key(password);

    encrypt(&key, message)
}
pub fn decrypt_message(password: &str, ciphertext: &Vec<u8>) -> Result<Vec<u8>, CliError> {
    let key = construct_key(password);

    decrypt(&key, ciphertext)
}
