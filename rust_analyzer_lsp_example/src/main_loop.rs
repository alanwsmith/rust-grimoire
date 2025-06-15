use crate::global_state::GlobalState;
use crate::notifications::handle_notification;
use crate::requests::handle_request;
use crate::responses::handle_response;
use lsp_server::{Connection, Message};
use std::error::Error;
use tracing::{Level, event};

pub fn main_loop(
  connection: Connection,
  _params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
  event!(Level::DEBUG, "Starting main loop");
  let mut global_state = GlobalState::new();

  for msg in &connection.receiver {
    match msg {
      Message::Request(message) => {
        if connection.handle_shutdown(&message)? {
          return Ok(());
        }
        handle_request(
          message,
          &connection,
          &global_state,
        );
      }

      Message::Response(message) => {
        handle_response(message);
      }

      Message::Notification(message) => {
        handle_notification(message, &mut global_state);
      }
    }
  }
  Ok(())
}
