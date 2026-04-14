use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("PROTOC", protobuf_src::protoc());

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let proto_dir = manifest_dir.join("protos");
    let proto_file = proto_dir.join("old-faithful.proto");

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile_protos(&[proto_file.clone()], &[proto_dir])?;

    println!("cargo:rerun-if-changed={}", proto_file.display());

    Ok(())
}
