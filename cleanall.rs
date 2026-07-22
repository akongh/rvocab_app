use std::fs;
use std::path::Path;

fn main() {
    let paths_to_remove = vec![
        "src/resource.res",
        "rvocab.html",
    ];

    for path in paths_to_remove {
        if Path::new(path).exists() {
            fs::remove_file(path).expect("Failed to remove file.");
        }
    }
}
