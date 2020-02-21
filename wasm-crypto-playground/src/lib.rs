use wasm_bindgen::prelude::*;
use std::str;

mod browser_crypto;

// globalThis.crypto.getRandomValues(new Uint8Array(5))

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let uuid = uuid();
    let randomuuid = str::from_utf8(&uuid).unwrap();

    alert(&format!("From WebCrypto:, {}!", randomuuid));
    //alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn get_random() {
    let name = "Ava";
    alert(&format!("Hello, {}!", name));
    //let uuid = uuid();
    //let randomuuid = str::from_utf8(&uuid).unwrap();

    //alert(&format!("From WebCrypto:, {}!", randomuuid));
}


pub fn uuid() -> Vec<u8> {
    let mut bytes = [0; 16];
    browser_crypto::crypto()
        .get_random_values_with_u8_array(&mut bytes)
        .unwrap();
    bytes[6] = bytes[6] & 0b0000_1111 | 0b0100_0000; // 0 1 0 0 x x x x
    bytes[8] = bytes[8] & 0b0011_1111 | 0b1000_0000; // 1 0 x x x x x x
    bytes.to_vec()
}