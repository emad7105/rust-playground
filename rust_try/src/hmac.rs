use sha2::Sha256;
use hmac::{Hmac, Mac};
use hex_literal::hex;

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

pub fn run() {
    let mut mac = HmacSha256::new_from_slice(b"my secret and secure key")
        .expect("HMAC can take key of any size");
    mac.update(b"input message");


// `result` has type `CtOutput` which is a thin wrapper around array of
// bytes for providing constant time equality check
    let result = mac.finalize();
// To get underlying array use `into_bytes`, but be careful, since
// incorrect use of the code value may permit timing attacks which defeats
// the security provided by the `CtOutput`
    let code_bytes = result.into_bytes();
    let expected = hex!("
    97d2a569059bbcd8ead4444ff99071f4
    c01d005bcefe0d3567e1be628e5fdcd9
");
    assert_eq!(code_bytes[..], expected[..]);
}