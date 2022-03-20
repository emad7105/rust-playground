use rsa::{PublicKey, RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rand::rngs::OsRng;

pub fn run() {
    let mut rng = OsRng;
    let (private_key, public_key) = gen_rsa_keypair(&mut rng);

    // Encrypt
    let data = b"hello world";
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let enc_data = public_key.encrypt(&mut rng, padding, &data[..]).expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);

    // Decrypt
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let dec_data = private_key.decrypt(padding, &enc_data).expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);
}

fn gen_rsa_keypair(mut rng: &mut OsRng) -> (RsaPrivateKey, RsaPublicKey) {
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}


