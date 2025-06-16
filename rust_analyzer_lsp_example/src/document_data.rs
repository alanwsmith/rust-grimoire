// This is pretty much a straight copy from
// the rust_analyzer code. I changed the data
// from bytes to a String for my use case.
//
// https://github.com/rust-lang/rust-analyzer/tree/master/crates/rust-analyzer/src

#[derive(Debug, Clone)]
pub struct DocumentData {
  pub version: i32,
  pub data: String,
}

impl DocumentData {
  pub fn new(
    version: i32,
    data: String,
  ) -> Self {
    DocumentData { version, data }
  }
}
