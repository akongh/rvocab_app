use std::fs;
use std::path::Path;

fn main() {
    let paths_to_remove = vec!["src/resource.res", "rvocab.html"];

    for path in paths_to_remove {
        // println!("{}", path);
        if Path::new(path).exists() {
            fs::remove_file(path).unwrap_or_else(|_| panic!("> Failed to remove file: {}.", path))
        } else {
            eprintln!("> File does not exist: {}.", path)
        }
    }
}
