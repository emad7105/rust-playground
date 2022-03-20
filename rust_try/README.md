# Cheat sheet




## Cargo

In a folder:  
`cargo init`

To compile and run:  
``cargo run``

To build:    
`cargo build`    

To build for production:  
 `cargo build --release`  

### Faster RNG
Taken from [RustCrypto/RSA](https://github.com/RustCrypto/RSA):  

> **Note:** If you encounter unusually slow key generation time while using `RsaPrivateKey::new` you can try to compile in release mode or add the following to your `Cargo.toml`. Key generation is much faster when building with higher optimization levels, but this will increase the compile time a bit.
> ```toml
> [profile.debug]
> opt-level = 3
> ```
> If you don't want to turn on optimizations for all dependencies,
> you can only optimize the `num-bigint-dig` dependency. This should
> give most of the speedups.
> ```toml
> [profile.dev.package.num-bigint-dig]
> opt-level = 3
> ```
