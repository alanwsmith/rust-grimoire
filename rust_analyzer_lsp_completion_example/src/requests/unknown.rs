use lsp_server::{Request, Response};
use tracing::{Level, event};

pub fn unknown_request(
  message: &Request
) -> Option<Response> {
  event!(
    Level::INFO,
    "Unknown Request: \n{:?}",
    message
  );
  None
}
