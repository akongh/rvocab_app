fn main() {
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-arg=src/resource.res");
        println!("cargo:rerun-if-changed=src/resource.res");
    }
}
