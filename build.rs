use std::path::Path;
use std::process::Command;

fn main() {
    if cfg!(target_os = "windows") {
        // Путь к файлам
        let rc_path = "src/resources/app.rc";
        let res_path = "src/resources/resource.res";

        // Компилируем app.rc в resource.res с помощью windres
        let windres_status = Command::new("windres")
            .args(&[rc_path, res_path])
            .status()
            .expect("Failed to run windres. Make sure it is installed and in PATH.");

        if !windres_status.success() {
            panic!("windres failed to compile the .rc file");
        }

        // Передаём resource.res линковщику
        println!("cargo:rustc-link-arg={}", res_path);
        println!("cargo:rerun-if-changed={}", rc_path);
    }
}
