extern crate bindgen;
extern crate napi_build;

use std::env;
use std::process;
use std::path::PathBuf;

fn envvar(name: &str) -> String {
  match env::var(name) {
    Ok(val) => val,
    Err(e) => {
      eprintln!("Unable to read {} {}", name, e);
      process::exit(1);
    }
  }
}

fn main() {
  const OUT_DIR: &str = "OUT_DIR";
  let pwd: String = envvar("PWD");
  let go_file_path_buf: PathBuf = PathBuf::from(pwd).join("src/readFile.go");
  let go_file: &str = go_file_path_buf.to_str().unwrap();
  const STATIC_LIB: &str = "ReadFile";
  gobuild::Build::new().file(go_file).compile(STATIC_LIB);
  let c_header_file_path_buf: PathBuf = PathBuf::from(envvar(OUT_DIR)).join("libReadFile.h");
  let c_header_file: &str = c_header_file_path_buf.to_str().unwrap();
  bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(c_header_file)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings")
        .write_to_file(PathBuf::from(envvar(OUT_DIR)).join("bindings.rs"))
        .expect("Couldn't write bindings!");
  println!("cargo:rerun-if-changed={}", go_file);
  napi_build::setup();
}
