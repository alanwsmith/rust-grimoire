pub mod completion;
pub mod unknown;

use crate::global_state::GlobalState;
use crate::requests::completion::completion;
use crate::requests::unknown::unknown_request;
use lsp_server::{Connection, Message};

pub fn handle_request(
  message: &lsp_server::Request,
  connection: &Connection,
  global_state: &GlobalState,
) {
  let response = match message.method.as_str() {
    "textDocument/completion" => {
      completion(&message, &global_state)
    }
    _ => unknown_request(&message),
  };

  match response {
    Some(r) => {
      let _ =
        connection.sender.send(Message::Response(r));
    }
    None => (),
  }
}
