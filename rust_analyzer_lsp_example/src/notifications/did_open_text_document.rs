use crate::notifications::cast::cast_notify;
use crate::{
  document_data::DocumentData,
  global_state::GlobalState,
};
use lsp_types::notification::DidOpenTextDocument;
use tracing::{Level, event};

pub fn did_open_text_document(
  message: lsp_server::Notification,
  global_state: &mut GlobalState,
) {
  match cast_notify::<DidOpenTextDocument>(message) {
    Ok(params) => {
      event!(Level::TRACE, "{:?}", &params);
      let uri = params.text_document.uri.to_string();
      let version = params.text_document.version;
      let text = params.text_document.text.to_string();
      let doc = DocumentData::new(version, text);
      let _ = global_state.mem_docs.insert(&uri, doc);
    }
    Err(e) => {
      event!(Level::ERROR, "{}", e);
    }
  }
}
