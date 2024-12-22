use aes_gcm::{
    aead::{AeadCore, AeadInPlace, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key,
};
use aes_gcm::aead::Aead;
use aes_gcm::aead::generic_array::typenum::U12;

type Keylen32u8 = [u8; 32];
pub type AesGcmKey<'a> = &'a Keylen32u8;

pub fn aes_gcm_key_from_string_literal(data: &[u8]) -> Keylen32u8 {
    assert_eq!(data.len(), 32, "Data must be exactly 32 bytes long");
    let mut key = [0u8; 32];
    key.copy_from_slice(data);
    key
}

pub fn encrypt(key: AesGcmKey,
               plaintext: &[u8],
               associated_data: &[u8],
) -> (Vec<u8>, aes_gcm::Nonce<aes_gcm::aead::generic_array::typenum::U12>) {
    let key: &Key<Aes256Gcm> = key.into();
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let mut buffer = plaintext.to_vec();
    let encrypted_bytes = cipher.encrypt(&nonce,
                                         aes_gcm::aead::Payload {
                                             msg: plaintext,
                                             aad: associated_data,
                                         }).expect("Encryption failed");
    (encrypted_bytes, nonce)
}

pub fn decrypt(key: AesGcmKey, nonce: &Nonce<U12>,
               ciphertext: &[u8], associated_data: &[u8]) -> Vec<u8> {
    let key: &Key<Aes256Gcm> = key.into();
    let cipher = Aes256Gcm::new(key);
    let decrypted_bytes = cipher
        .decrypt(nonce,
                 aes_gcm::aead::Payload {
                     msg: ciphertext,
                     aad: associated_data,
                 },
        )
        .expect("Decryption failed");
    decrypted_bytes
}


pub fn test_encrypt_and_decrypt() {
    let associated_data = b"sksks";
    let data = b"jsjsjs";
    let key  = aes_gcm_key_from_string_literal(b"0123456789abcdef0123456789abcdef");
    println!("Original data: {:?}", data);
    let (ciphered, nonce) =  encrypt(&key,   data, b"sksks");
    println!("Encrypted data: {:?}", ciphered);
    let plain_text = decrypt(&key, &nonce, &ciphered, associated_data);
    println!("Decrypted data: {:?}", plain_text);
}
//