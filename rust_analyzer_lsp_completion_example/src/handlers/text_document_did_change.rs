use crate::casts::*;
use crate::document_data::DocumentData;
use crate::mem_docs::MemDocs;
use lsp_types::notification::DidChangeTextDocument;
// use tracing::{Level, event};

pub fn text_document_did_change(
  mem_docs: &mut MemDocs,
  notif: lsp_server::Notification,
) {
  match cast_notify::<DidChangeTextDocument>(notif) {
    Ok(params) => {
      // event!(Level::INFO, "{:?}", params);
      let uri = params.text_document.uri.to_string();
      let version = params.text_document.version;
      let text =
        params.content_changes[0].text.to_string();
      let _ = mem_docs
        .insert(&uri, DocumentData::new(version, text));
    }
    Err(e) => (),
  }
}
