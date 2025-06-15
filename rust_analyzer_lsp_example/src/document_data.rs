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
