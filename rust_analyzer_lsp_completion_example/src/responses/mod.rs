pub mod unknown;

use crate::responses::unknown::unknown_response;
use lsp_server::Response;

pub fn handle_response(message: &Response) {
  unknown_response(&message);
}
