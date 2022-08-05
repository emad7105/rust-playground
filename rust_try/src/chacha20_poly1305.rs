use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce}; // Or `XChaCha20Poly1305`
use chacha20poly1305::aead::{Aead, NewAead};

use rand::{RngCore, rngs};

pub fn run() {
    let key = Key::from_slice(b"an example very very secret key."); // 32-bytes
    let cipher = ChaCha20Poly1305::new(key);

    let nonce = Nonce::from_slice(b"unique nonce"); // 12-bytes; unique per message

    let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref())
        .expect("encryption failure!");  // NOTE: handle this error to avoid panics!
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!");  // NOTE: handle this error to avoid panics!

    assert_eq!(&plaintext, b"plaintext message");

    // gen key
    let key_256 = gen_key_256bit();
    let key_hex = hex::encode(key_256);
    println!("{:?}", key_hex);
}

pub fn gen_key_256bit() -> Vec<u8> {
    // generate a key
    let mut key: [u8; 32] = [0; 32]; // 256-bits
    let mut os_rng = rngs::OsRng::default();
    os_rng.fill_bytes(&mut key);
    key.to_vec()
}





