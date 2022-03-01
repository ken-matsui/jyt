use std::env;
use std::path::PathBuf;

fn main() {
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut target_dir = PathBuf::from(manifest_dir.clone());
    target_dir.push("target");
    target_dir.push(env::var("PROFILE").unwrap());

    let mut header_path = target_dir.clone();
    header_path.push(format!("{}.h", pkg_name));
    cbindgen::Builder::new()
        .with_crate(&manifest_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .unwrap()
        .write_to_file(header_path);

    let include_and_shared_object_dir = target_dir.as_path().to_string_lossy();
    println!(
        "cargo:rustc-env=INLINE_C_RS_CFLAGS=-I{I} -L{L} -D_DEBUG -D_CRT_SECURE_NO_WARNINGS",
        I = include_and_shared_object_dir,
        L = include_and_shared_object_dir,
    );

    let shared_object_name = format!("lib{}.dylib", pkg_name.replace('-', "_"));
    println!(
        "cargo:rustc-env=INLINE_C_RS_LDFLAGS={shared_object_dir}/{lib}",
        shared_object_dir = include_and_shared_object_dir,
        lib = shared_object_name,
    );
}
