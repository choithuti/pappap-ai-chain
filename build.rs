// build.rs
fn main() {
    // Compile file Protobuf cho module Genetics
    prost_build::compile_protos(&["genetics/system_dna.proto"], &["genetics/"])
        .expect("❌ Failed to compile System DNA Protos");
}
