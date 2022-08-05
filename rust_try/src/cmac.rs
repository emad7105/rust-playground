use aes::Aes128;
use cmac::{Cmac, Mac};

pub fn run() {

// Create `Mac` trait implementation, namely CMAC-AES128
    let mut mac = Cmac::<Aes128>::new_from_slice(b"very secret key.").unwrap();
    mac.update(b"input message");

// `result` has type `Output` which is a thin wrapper around array of
// bytes for providing constant time equality check
    let result = mac.finalize();
// To get underlying array use the `into_bytes` method, but be careful,
// since incorrect use of the tag value may permit timing attacks which
// defeat the security provided by the `Output` wrapper
    let tag_bytes = result.into_bytes();
}