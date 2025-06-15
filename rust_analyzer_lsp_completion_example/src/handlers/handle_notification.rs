use crate::{
  global_state::GlobalState,
  handlers::unknown_notification::unknown_notification,
};
use lsp_server::{Connection, Notification};

pub fn handle_notification(
  message: &Notification,
  _connection: &Connection,
  _global_state: &GlobalState,
) {
  match message.method.as_str() {
    "textDocument/didChange" => {
      //text_document_did_change(&mut mem_docs, message)
    }
    _ => unknown_notification(&message),
  }
}
