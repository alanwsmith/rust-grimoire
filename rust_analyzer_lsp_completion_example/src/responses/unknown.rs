use lsp_server::Response;
use tracing::{Level, event};

pub fn unknown_response(message: &Response) {
  event!(
    Level::INFO,
    "Unknown Response\n{:?}",
    message
  );
}
