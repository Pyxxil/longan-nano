use std::path::PathBuf;
use std::{env, fs};

fn main() {
    // Put the memory definitions somewhere the linker can find it
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());

    //fs::copy("memory-c8.x", out_dir.join("memory-c8.x")).unwrap();
    //println!("cargo:rerun-if-changed=memory-c8.x");

    fs::copy("memory-cb.x", out_dir.join("memory-cb.x")).unwrap();
    println!("cargo:rerun-if-changed=memory-cb.x");
}
