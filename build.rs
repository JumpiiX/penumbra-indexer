use std::path::PathBuf;

fn main() {
    let _proto_dir = PathBuf::from("proto");  // Added underscore to silence warning
    println!("cargo:rerun-if-changed=proto/");

    tonic_build::configure()
        .build_server(true)
        .compile(
            &["proto/compact_block.proto"],
            &["proto"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));
}