use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Path to the `inputs` folder (relative to the project root)
    let inputs_path = Path::new("inputs");

    // Ensure the `inputs` folder exists
    if !inputs_path.exists() {
        panic!("The `inputs` folder does not exist!");
    }

    // Get the output directory where the executable is built
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR environment variable not set");

    // Construct the destination path for the `inputs` folder
    let out_dir_path = Path::new(&out_dir).join("../../../inputs");

    // Copy the folder (recursively) to the target directory
    if let Err(e) = copy_dir_all(inputs_path, &out_dir_path) {
        panic!("Failed to copy `inputs` folder: {}", e);
    }

    println!("cargo:rerun-if-changed=inputs");
}

// Function to copy a directory recursively
fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir_all(&path, &dest_path)?;
        } else {
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}
