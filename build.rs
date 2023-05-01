extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=/usr/local/lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=rpitx");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.hpp");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .rustfmt_bindings(true)
        .whitelist_type(r".*gpio")
        .whitelist_type(r".*dma.*")
        .whitelist_type("page_map_t")
        .derive_debug(true)
        .impl_debug(true)
        .clang_arg("-I/opt/vc/include")
        .clang_arg("-I/usr/arm-linux-gnueabihf/include/")
        .clang_arg("-I/usr/lib/gcc/arm-linux-gnueabihf/4.6/include-fixed/")
        .clang_arg("-I/usr/lib/gcc/arm-linux-gnueabihf/4.6/include/")
        // bindings for.
        .header("wrapper.hpp")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=rt");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=bcm_host");
    println!("cargo:rustc-link-lib=supc++");
    println!("cargo:rustc-link-search=native=/opt/vc/lib");
    println!("cargo:rustc-link-search=/opt/vc/lib");
}
