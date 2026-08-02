use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let bin_name = env::var("CARGO_BIN_NAME").unwrap_or_default();

    if bin_name.is_empty() && cfg!(target_os = "windows") {
            let rc_path = Path::new("src/resource.rc");
            let res_path = Path::new("src/resource.res");

            if !rc_path.exists() {
                panic!("File {} does not exist.", rc_path.display());
            }

            let windres_status = Command::new("windres")
                .args([rc_path.to_str().unwrap(), res_path.to_str().unwrap()])
                .status()
                .expect("Failed to run windres. Make sure it is installed and in PATH.");

            if !windres_status.success() {
                panic!("windres failed to compile the resource.rc file.");
            }

            println!("cargo:rustc-link-arg={}", res_path.display());
            println!("cargo:rerun-if-changed={}", rc_path.display());
        }
    }
