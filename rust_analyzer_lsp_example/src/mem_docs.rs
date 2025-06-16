use crate::document_data::DocumentData;
use rustc_hash::FxHashMap;
use std::mem;

// This is pretty much a straight copy from
// the rust_analyzer code:
//
// https://github.com/rust-lang/rust-analyzer/tree/master/crates/rust-analyzer/src

#[derive(Default, Clone)]
pub struct MemDocs {
  mem_docs: FxHashMap<String, DocumentData>,
  added_or_removed: bool,
}

impl MemDocs {
  pub fn contains(
    &self,
    path: &str,
  ) -> bool {
    self.mem_docs.contains_key(path)
  }

  pub fn insert(
    &mut self,
    path: &str,
    data: DocumentData,
  ) -> Result<(), ()> {
    self.added_or_removed = true;
    match self.mem_docs.insert(path.to_string(), data) {
      Some(_) => Err(()),
      None => Ok(()),
    }
  }

  pub fn remove(
    &mut self,
    path: &str,
  ) -> Result<(), ()> {
    self.added_or_removed = true;
    match self.mem_docs.remove(path) {
      Some(_) => Ok(()),
      None => Err(()),
    }
  }

  pub fn get(
    &self,
    path: &str,
  ) -> Option<&DocumentData> {
    self.mem_docs.get(path)
  }

  pub fn get_mut(
    &mut self,
    path: &str,
  ) -> Option<&mut DocumentData> {
    // NB: don't set `self.added_or_removed` here, as that purposefully only
    // tracks changes to the key set.
    self.mem_docs.get_mut(path)
  }

  pub fn iter(&self) -> impl Iterator<Item = &String> {
    self.mem_docs.keys()
  }

  pub fn take_changes(&mut self) -> bool {
    mem::replace(&mut self.added_or_removed, false)
  }
}
