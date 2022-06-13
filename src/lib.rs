use napi::bindgen_prelude::Result;
use napi_derive::napi;
use std::ffi::CString;
use std::os::raw::c_char;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// #[link(name = "ReadFile", kind = "static")]
// extern "C" {
//   pub fn ReadTextFileSize(path: *const c_char) -> i32;
// }

#[napi]
async fn get_file_size(path: String) -> Result<i32> {
  let p = CString::new(path).expect("CString::new failed");
  let size = unsafe { ReadTextFileSize(p.into_raw()) };
  Ok(size)
}

#[napi]
fn sum(a: i32, b: i32) -> i32 {
  a + b
}
