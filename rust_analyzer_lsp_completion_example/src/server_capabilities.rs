use lsp_types::ServerCapabilities;
use lsp_types::TextDocumentSyncOptions;
use serde_json::Value;
use tracing::{Level, event};

pub fn server_capabilities() -> Value {
  event!(Level::DEBUG, "Defining server capabilities");
  serde_json::to_value(&ServerCapabilities {
    text_document_sync: Some(
      lsp_types::TextDocumentSyncCapability::Options(
        TextDocumentSyncOptions {
          change: Some(
            lsp_types::TextDocumentSyncKind::FULL,
          ),
          open_close: Some(false),
          will_save: Some(false),
          will_save_wait_until: Some(false),
          save: None,
        },
      ),
    ),
    completion_provider: Some(
      lsp_types::CompletionOptions {
        trigger_characters: Some(vec!["-".to_string()]),
        ..Default::default()
      },
    ),
    ..Default::default()
  })
  .expect("Could not set up server capabilities")
}

// lsp_types::TextDocumentSyncCapability::Kind(
//   TextDocumentSyncKind::FULL,
// ),
