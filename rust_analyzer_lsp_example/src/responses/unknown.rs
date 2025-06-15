use lsp_server::Response;
use tracing::{Level, event};

pub fn unknown_response(message: Response) {
  event!(Level::ERROR, "\n\n{:?}", message);
}
