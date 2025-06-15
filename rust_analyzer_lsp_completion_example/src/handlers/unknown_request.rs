use lsp_server::Request;
use tracing::{Level, event};

pub fn unknown(message: &Request) {
  event!(
    Level::INFO,
    "Unknown Request: \n{:?}",
    message
  );
}
