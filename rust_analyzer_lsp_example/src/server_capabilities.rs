use lsp_types::SemanticTokenModifier;
use lsp_types::SemanticTokenType;
use lsp_types::SemanticTokensFullOptions;
use lsp_types::SemanticTokensLegend;
use lsp_types::SemanticTokensOptions;
use lsp_types::ServerCapabilities;
use lsp_types::TextDocumentSyncOptions;
use serde_json::Value;
use tracing::{Level, event};

pub fn server_capabilities() -> Value {
  event!(Level::DEBUG, "Defining server capabilities");
  serde_json::to_value(&ServerCapabilities {
    completion_provider: Some(
      lsp_types::CompletionOptions {
        trigger_characters: Some(vec!["-".to_string()]),
        ..Default::default()
      },
    ),
    document_formatting_provider: Some(
      lsp_types::OneOf::Left(true),
    ),

    // Semantic tokens are a work in progress
    semantic_tokens_provider: Some(
      SemanticTokensOptions {
        legend: SemanticTokensLegend {
          token_types: vec![
            SemanticTokenType::CLASS,
            SemanticTokenType::COMMENT,
            SemanticTokenType::DECORATOR,
            SemanticTokenType::ENUM,
            SemanticTokenType::ENUM_MEMBER,
            SemanticTokenType::EVENT,
            SemanticTokenType::FUNCTION,
            SemanticTokenType::INTERFACE,
            SemanticTokenType::KEYWORD,
            SemanticTokenType::MACRO,
            SemanticTokenType::METHOD,
            SemanticTokenType::MODIFIER,
            SemanticTokenType::NAMESPACE,
            SemanticTokenType::NUMBER,
            SemanticTokenType::OPERATOR,
            SemanticTokenType::PARAMETER,
            SemanticTokenType::PROPERTY,
            SemanticTokenType::REGEXP,
            SemanticTokenType::STRING,
            SemanticTokenType::STRUCT,
            SemanticTokenType::TYPE,
            SemanticTokenType::TYPE_PARAMETER,
            SemanticTokenType::VARIABLE,
          ],
          token_modifiers: vec![
            SemanticTokenModifier::ABSTRACT,
            SemanticTokenModifier::ASYNC,
            SemanticTokenModifier::DECLARATION,
            SemanticTokenModifier::DEFAULT_LIBRARY,
            SemanticTokenModifier::DEFINITION,
            SemanticTokenModifier::DEPRECATED,
            SemanticTokenModifier::DOCUMENTATION,
            SemanticTokenModifier::MODIFICATION,
            SemanticTokenModifier::READONLY,
            SemanticTokenModifier::STATIC,
          ],
        },
        full: Some(SemanticTokensFullOptions::Bool(
          true,
        )),
        range: None,
        work_done_progress_options:
          lsp_types::WorkDoneProgressOptions {
            work_done_progress: None,
          },
      }
      .into(),
    ),

    text_document_sync: Some(
      lsp_types::TextDocumentSyncCapability::Options(
        TextDocumentSyncOptions {
          change: Some(
            lsp_types::TextDocumentSyncKind::FULL,
          ),
          open_close: Some(true),
          will_save: None,
          will_save_wait_until: None,
          save: None,
        },
      ),
    ),
    ..Default::default()
  })
  .expect("Could not set up server capabilities")
}
