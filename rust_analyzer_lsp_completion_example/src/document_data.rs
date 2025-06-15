#[derive(Debug, Clone)]
pub struct DocumentData {
  pub(crate) version: i32,
  pub(crate) data: String,
}

impl DocumentData {
  pub fn new(
    version: i32,
    data: String,
  ) -> Self {
    DocumentData { version, data }
  }
}
