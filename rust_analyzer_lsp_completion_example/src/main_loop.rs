use crate::{
  global_state::GlobalState,
  handlers::handle_request::handle_request,
};
use lsp_server::{Connection, Message};
use lsp_types::InitializeParams;
use std::error::Error;
use tracing::{Level, event};

pub fn main_loop(
  connection: Connection,
  params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
  event!(Level::INFO, "Starting main loop");
  let mut global_state = GlobalState::new();

  let _params: InitializeParams =
    serde_json::from_value(params).unwrap();

  for msg in &connection.receiver {
    match msg {
      Message::Request(req) => {
        if connection.handle_shutdown(&req)? {
          return Ok(());
        }
        handle_request(
          &req,
          &connection,
          &global_state,
        );
      }

      Message::Response(resp) => {
        event!(Level::INFO, "Got response: {resp:?}");
      }

      Message::Notification(notif) => {}
    }
  }
  Ok(())
}
