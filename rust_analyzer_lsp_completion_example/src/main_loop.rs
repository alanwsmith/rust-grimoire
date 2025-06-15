use crate::{
  global_state::GlobalState,
  handlers::{
    handle_notification::handle_notification,
    handle_request::handle_request,
    handle_response::handle_response,
  },
};
use lsp_server::{Connection, Message};
use lsp_types::InitializeParams;
use std::error::Error;
use tracing::{Level, event};

pub fn main_loop(
  connection: Connection,
  params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
  event!(Level::DEBUG, "Starting main loop");
  let mut global_state = GlobalState::new();

  let _params: InitializeParams =
    serde_json::from_value(params).unwrap();

  for msg in &connection.receiver {
    match msg {
      Message::Request(message) => {
        if connection.handle_shutdown(&message)? {
          return Ok(());
        }
        handle_request(
          &message,
          &connection,
          &global_state,
        );
      }

      Message::Response(message) => {
        handle_response(
          &message,
          &connection,
          &global_state,
        );
      }

      Message::Notification(message) => {
        handle_notification(
          &message,
          &connection,
          &global_state,
        );
      }
    }
  }
  Ok(())
}
