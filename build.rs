// build.rs
use std::io::Result;

fn main() -> Result<()> {
    // Chỉ định biên dịch file system_dna.proto trong thư mục genetics/
    prost_build::compile_protos(&["genetics/system_dna.proto"], &["genetics/"])?;
    Ok(())
}
