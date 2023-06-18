use std::fs::create_dir_all;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    create_dir_all(Path::new("src/proto"))?;
    tonic_build::configure().out_dir("src/proto").compile(
        &["proto/UserService.proto", "proto/NotifyService.proto"],
        &["proto/"],
    )?;
    Ok(())
}
