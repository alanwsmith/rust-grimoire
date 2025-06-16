use lsp_server::Notification;
use tracing::{Level, event};

pub fn unknown_notification(message: &Notification) {
  event!(Level::ERROR, "{:?}", message);
}
