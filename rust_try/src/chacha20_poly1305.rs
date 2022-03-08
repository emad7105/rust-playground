use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce}; // Or `XChaCha20Poly1305`
use chacha20poly1305::aead::{Aead, NewAead};

pub fn run() {
    let key = Key::from_slice(b"an example very very secret key."); // 32-bytes
    let cipher = ChaCha20Poly1305::new(key);

    let nonce = Nonce::from_slice(b"unique nonce"); // 12-bytes; unique per message

    let ciphertext = cipher.encrypt(nonce, b"plaintext message".as_ref())
        .expect("encryption failure!");  // NOTE: handle this error to avoid panics!
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
        .expect("decryption failure!");  // NOTE: handle this error to avoid panics!

    assert_eq!(&plaintext, b"plaintext message");
}




