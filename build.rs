use std::env;
use std::fs::{self, create_dir_all};
use std::path::Path;

fn main() {
    // Tell Cargo to rerun if resources/ changes
    println!("cargo:rerun-if-changed=resources");

    // Get project root and profile (debug or release)
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let profile = env::var("PROFILE").expect("PROFILE not set");

    // Paths for source and destination
    let src_resources = Path::new(&manifest_dir).join("resources");
    let dest_resources = Path::new(&manifest_dir)
        .join("target")
        .join(profile)
        .join("resources");

    // Remove existing resources/ in target/<profile>/
    if dest_resources.exists() {
        fs::remove_dir_all(&dest_resources).expect("Failed to remove old resources");
    }

    // Create destination directory
    create_dir_all(&dest_resources.parent().unwrap()).expect("Failed to create target directory");

    // Copy resources/ to target/<profile>/resources/
    copy_dir_recursive(&src_resources, &dest_resources).expect("Failed to copy resources");
}

// Recursive directory copy function
fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !src.exists() {
        return Ok(());
    }
    create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
