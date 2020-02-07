## Your Application
In the `cargo.toml` file, we should have:  
```
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2.11"
```

Now you need to run `cargo build` to fetch and build all dependecies.
 
After placing codes, we need to build the code to the WASM as follows:  
```
cargo +nightly build --target wasm32-unknown-unknown
```

The output of this build is a folder which contains a `.wasm` file. This file is huge and not necessarily optimised. We can use `bindgen` cli tool to fix this.
```
wasm-bindgen target/wasm32-unknown-unknown/bug/wasm_example.wasm --out-dir .
```




## Requirements
Adding WASM to our compiler:  
  
```
rustup target add wasm32-unknown-unknown
cargo +nightly install wasm-bindgen-cli
```  

Compiling to WASM:  
```
cargo +nightly new wasm_example --lib
```
