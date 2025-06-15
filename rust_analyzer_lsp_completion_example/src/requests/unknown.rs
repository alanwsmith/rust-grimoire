use lsp_server::{Request, Response};
use tracing::{Level, event};

pub fn unknown_request(message: Request) -> Response {
  event!(Level::ERROR, "\n\n{:?}", message);
  Response {
    id: message.id,
    result: None,
    // TODO: Figure out what error to send back
    error: None,
  }
}
