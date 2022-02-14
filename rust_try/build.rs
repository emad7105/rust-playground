/*fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("protos/helloworld.proto")?;
    Ok(())
}*/

use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("helloworld_descriptor.bin"))
        .compile(&["protos/helloworld.proto"], &["protos"])
        .unwrap();
}
