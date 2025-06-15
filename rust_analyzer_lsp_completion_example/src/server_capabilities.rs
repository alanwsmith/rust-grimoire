use lsp_types::ServerCapabilities;
use lsp_types::TextDocumentSyncKind;
use serde_json::Value;
use tracing::{Level, event};

pub fn server_capabilities() -> Value {
  event!(Level::INFO, "Defining server capabilities");
  serde_json::to_value(&ServerCapabilities {
    text_document_sync: Some(
      lsp_types::TextDocumentSyncCapability::Kind(
        TextDocumentSyncKind::FULL,
      ),
    ),
    completion_provider: Some(
      lsp_types::CompletionOptions {
        ..Default::default()
      },
    ),
    ..Default::default()
  })
  .expect("Could not set up server capabilities")
}
