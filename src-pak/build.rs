use std::path::Path;

fn main() {
    let dir_current = std::env::current_dir().unwrap();
    let dir_lib = dir_current.join("lib");
    let dir_repo = Path::new("src/jx3pak");
    if dir_repo.exists() {
        println!("cargo:rustc-cfg=feature=\"lib_local\"");
        return;
    }

    let import = if cfg!(target_os = "windows") {
        "pak.dll"
    } else {
        "pak"
    };
    println!("cargo:rustc-link-lib=dylib={}", import);
    println!("cargo:rustc-link-search=native={}", dir_lib.display());
    // if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
    //     println!("cargo:rustc-link-arg=-Wl,-rpath,./");
    // }
    // Copy dynamic library to OUT_DIR
    let dir_out = std::env::var("OUT_DIR").unwrap();
    let dir_target = Path::new(&dir_out);
    let dir_target = dir_target.parent().unwrap();
    let dir_target = dir_target.parent().unwrap();
    let dir_target = dir_target.parent().unwrap();
    let fn_lib = if cfg!(target_os = "windows") {
        "pak.dll"
    } else if cfg!(target_os = "macos") {
        "libpak.dylib"
    } else {
        "libpak.so"
    };
    let path_src = dir_lib.join(fn_lib);
    let path_dest = dir_target.join(fn_lib);
    std::fs::copy(&path_src, &path_dest).unwrap();
}
