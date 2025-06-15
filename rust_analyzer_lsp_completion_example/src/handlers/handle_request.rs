use crate::{
  global_state::GlobalState,
  handlers::completion::completion,
};
use lsp_server::{Connection, Message};
use tracing::{Level, event};

pub fn handle_request(
  req: &lsp_server::Request,
  conn: &Connection,
  global_state: &GlobalState,
) {
  event!(Level::INFO, "Got request: {req:?}");
  let response = match req.method.as_str() {
    "textDocument/completion" => {
      completion(&req, &global_state)
    }
    _ => {
      event!(Level::INFO, "Unhandled request type");
      None
    }
  };

  match response {
    Some(r) => {
      let _ = conn.sender.send(Message::Response(r));
    }
    None => (),
  }
}
