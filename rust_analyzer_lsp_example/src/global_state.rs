use crate::mem_docs::MemDocs;

pub struct GlobalState {
  pub mem_docs: MemDocs,
}

impl GlobalState {
  pub fn new() -> GlobalState {
    GlobalState {
      mem_docs: MemDocs::default(),
    }
  }
}
