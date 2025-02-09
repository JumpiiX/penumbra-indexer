use std::path::PathBuf;

fn main() {
    let proto_dir = PathBuf::from("proto");
    println!("cargo:rerun-if-changed=proto/");

    tonic_build::configure()
        .build_server(true)
        .compile(
            &["proto/compact_block.proto"],
            &["proto"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));
}