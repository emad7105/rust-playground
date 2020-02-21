use wasm_bindgen::JsCast;

pub fn crypto() -> web_sys::Crypto {
    return js_sys::global().crypto().unwrap();
    /*js_sys::global()
        .dyn_into::<web_sys::WorkerGlobalScope>()
        .unwrap()
        .crypto()
        .unwrap()*/
}