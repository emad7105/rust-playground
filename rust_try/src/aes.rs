extern crate rand;

//use rand::{ Rng, OsRng };
use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce};
// Or `Aes128GcmSiv`
use aes_gcm_siv::aead::{Aead, NewAead};

pub fn run() {
    // Key generation
    // key generation using RngOS in the tests below

    let key = Key::from_slice(b"an example very very secret key.");
    let cipher = Aes256GcmSiv::new(key);

    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

    let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref())
        .expect("encryption failure!");  // NOTE: handle this error to avoid panics!


    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!");  // NOTE: handle this error to avoid panics!

    assert_eq!(&plaintext, b"plaintext message");
}


#[cfg(test)]
mod test_aes {
    use rand::{RngCore, rngs};
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let key = Key::from_slice(b"an example very very secret key.");
        let cipher = Aes256GcmSiv::new(key);

        let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

        let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref())
            .expect("encryption failure!");  // NOTE: handle this error to avoid panics!


        let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
            .expect("decryption failure!");  // NOTE: handle this error to avoid panics!

        assert_eq!(&plaintext, b"plaintext message");
    }

    #[test]
    fn test_encrypt_decrypt_with_genrated_key_materials() {
        // generate key and nonce
        let mut key: [u8; 32] = [0; 32]; // 256-bits
        let mut nonce: [u8; 12] = [0; 12]; // 96-bits

        //let mut rng = Rng::new().ok().unwrap();
        let mut os_rng = rngs::OsRng::default();
        os_rng.fill_bytes(&mut key);
        os_rng.fill_bytes(&mut nonce);

        assert_ne!(key, [0; 32]);
        assert_ne!(nonce, [0; 12]);

        let aes_key = Key::from_slice(&key);
        let cipher = Aes256GcmSiv::new(aes_key);

        let aes_nonce = Nonce::from_slice(&nonce); // 96-bits; unique per message

        let ciphertext = cipher.encrypt(aes_nonce, b"plaintext message".as_ref())
            .expect("encryption failure!");  // NOTE: handle this error to avoid panics!


        let plaintext = cipher.decrypt(aes_nonce, ciphertext.as_ref())
            .expect("decryption failure!");  // NOTE: handle this error to avoid panics!

        assert_eq!(&plaintext, b"plaintext message");
    }
}

