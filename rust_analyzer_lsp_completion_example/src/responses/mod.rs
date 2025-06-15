pub mod unknown;

use crate::global_state::GlobalState;
use crate::responses::unknown::unknown_response;

use lsp_server::{Connection, Response};

pub fn handle_response(
  message: &Response,
  _connection: &Connection,
  _global_state: &GlobalState,
) {
  unknown_response(&message);
}
