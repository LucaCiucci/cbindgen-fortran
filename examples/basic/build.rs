
use std::env;

use cbindgen_fortran::Fortran;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen_fortran::cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_custom_language(Fortran::new("rust_module"))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("bindings.f90");
}