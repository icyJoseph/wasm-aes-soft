use aes_soft::block_cipher_trait::generic_array::GenericArray;
use aes_soft::block_cipher_trait::BlockCipher;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher_key = GenericArray::from_slice(key);
    let cipher = aes_soft::Aes128::new(cipher_key);
    let mut block = GenericArray::clone_from_slice(data);

    cipher.encrypt_block(&mut block);

    let encrypted = block.to_vec();
    return encrypted;
}

#[wasm_bindgen]
pub fn decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    let cipher_key = GenericArray::from_slice(key);
    let cipher = aes_soft::Aes128::new(cipher_key);
    let mut block = GenericArray::clone_from_slice(data);

    cipher.decrypt_block(&mut block);

    let decrypted = block.to_vec();
    return decrypted;
}
