use wasm_bindgen::prelude::*;
use std::str;

mod browser_crypto;

// globalThis.crypto.getRandomValues(new Uint8Array(5))

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

// Next let's define a macro that's like `println!`, only it works for
// `console.log`. Note that `println!` doesn't actually work on the wasm target
// because the standard library currently just eats all output. To get
// `println!`-like behavior in your app you'll likely want a macro like this.

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    //let uuid = uuid();
   // let randomuuid = str::from_utf8(&uuid).unwrap();

    alert(&format!("From WebCrypto:, {}!", name));
    //alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn lets_log() {
    console_log!("Hello from {}", "Emad");
}

#[wasm_bindgen]
pub fn get_random() {
    let name = "Ava";
    alert(&format!("Hello, {}!", name));
    //let uuid = uuid();
    //let randomuuid = str::from_utf8(&uuid).unwrap();

    //alert(&format!("From WebCrypto:, {}!", randomuuid));
}

/*
pub fn uuid() -> Vec<u8> {
    let mut bytes = [0; 16];
    browser_crypto::crypto()
        .get_random_values_with_u8_array(&mut bytes)
        .unwrap();
    bytes[6] = bytes[6] & 0b0000_1111 | 0b0100_0000; // 0 1 0 0 x x x x
    bytes[8] = bytes[8] & 0b0011_1111 | 0b1000_0000; // 1 0 x x x x x x
    bytes.to_vec()
}*/