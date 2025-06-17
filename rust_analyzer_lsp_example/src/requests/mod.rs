pub mod cast;
pub mod completion;
pub mod formatting;
pub mod semantic_tokens_full;
pub mod unknown;

use crate::global_state::GlobalState;
use crate::requests::completion::completion;
use crate::requests::formatting::formatting;
use crate::requests::unknown::unknown_request;
use lsp_server::{Connection, Message};
use semantic_tokens_full::semantic_tokens_full;

pub fn handle_request(
  message: lsp_server::Request,
  connection: &Connection,
  global_state: &GlobalState,
) {
  let response = match message.method.as_str() {
    "textDocument/completion" => {
      completion(message, &global_state)
    }
    "textDocument/formatting" => {
      formatting(message, &global_state)
    }
    "textDocument/semanticTokens/full" => {
      semantic_tokens_full(message, &global_state)
    }
    _ => unknown_request(message),
  };
  let _ =
    connection.sender.send(Message::Response(response));
}
