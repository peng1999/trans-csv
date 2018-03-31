#![feature(nll)]

extern crate bindgen;
extern crate cc;
extern crate glob;

use std::env;
use std::path::PathBuf;

fn main() {
    let files = glob::glob("sundown/sundown/*.c")
        .unwrap()
        .map(|s| s.unwrap());

    cc::Build::new()
        .file("sundown/wrapper.c")
        .files(files)
        .compile("sundown");

    // println!("cargo:rustc-link-lib=sundown");
    // println!("cargo:rustc-link-search={}");

    let bindings = bindgen::Builder::default()
        .header("sundown/wrapper.h")
        .whitelist_type("buf")
        .whitelist_function("render")
        .whitelist_function("bufrelease")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
