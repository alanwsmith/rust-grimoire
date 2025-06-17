use crate::global_state::GlobalState;
use crate::notifications::cast::cast_notify;
use lsp_types::notification::DidCloseTextDocument;
use tracing::{Level, event};

pub fn did_close_text_document(
  message: lsp_server::Notification,
  global_state: &mut GlobalState,
) {
  match cast_notify::<DidCloseTextDocument>(message) {
    Ok(params) => {
      event!(Level::TRACE, "{:?}", &params);
      let uri = params.text_document.uri.to_string();
      let _ = global_state.mem_docs.remove(&uri);
    }
    Err(e) => {
      event!(Level::ERROR, "{}", e);
    }
  }
}
