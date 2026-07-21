use std::path::Path;
use std::process::Command;

fn main() {
    if cfg!(target_os = "windows") {
        let rc_path = Path::new("src/resources.rc");
        let res_path = Path::new("src/resource.res");

        if !rc_path.exists() {
            panic!("File {} does not exist.", rc_path.display());
        }

        let windres_status = Command::new("windres")
            .args(&[rc_path.to_str().unwrap(), res_path.to_str().unwrap()])
            .status()
            .expect("Failed to run windres. Make sure it is installed and in PATH.");

        if !windres_status.success() {
            panic!("windres failed to compile the .rc file.");
        }

        println!("cargo:rustc-link-arg={}", res_path.display());
        println!("cargo:rerun-if-changed={}", rc_path.display());
    }
}
