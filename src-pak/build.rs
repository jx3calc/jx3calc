use std::path::Path;

fn main() {
    let lib_dir = Path::new(file!()).parent().unwrap().join("lib");
    let repo_dir = Path::new("src/jx3pak");
    if repo_dir.exists() {
        println!("cargo:rustc-cfg=feature=\"lib_local\"");
    } else {
        println!("cargo:rustc-link-lib=dylib=pak");
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-env=DYLD_LIBRARY_PATH={}", lib_dir.display());
    }
}
