pub mod cast;
pub mod did_change_text_document;
pub mod unknown;

use crate::global_state::GlobalState;
use crate::notifications::did_change_text_document::did_change_text_document;
use crate::notifications::unknown::unknown_notification;
use lsp_server::Notification;

pub fn handle_notification(
  message: Notification,
  global_state: &mut GlobalState,
) {
  match message.method.as_str() {
    "textDocument/didChange" => {
      did_change_text_document(message, global_state)
    }
    _ => unknown_notification(&message),
  }
}
