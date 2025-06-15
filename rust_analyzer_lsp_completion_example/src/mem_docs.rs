//! In-memory document information.

use rustc_hash::FxHashMap;
use std::mem;
// use vfs::VfsPath;

/// Holds the set of in-memory documents.
///
/// For these document, their true contents is maintained by the client. It
/// might be different from what's on disk.
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

/// Information about a document that the Language Client
/// knows about.
/// Its lifetime is driven by the textDocument/didOpen and textDocument/didClose
/// client notifications.
#[derive(Debug, Clone)]
pub struct DocumentData {
  pub(crate) version: i32,
  // pub(crate) data: Vec<u8>,
  pub(crate) data: String,
}

impl DocumentData {
  pub fn new(
    version: i32,
    //data: Vec<u8>,
    data: String,
  ) -> Self {
    DocumentData { version, data }
  }
}
