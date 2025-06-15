#![allow(unused)]
use crate::handle_completion::*;
use crate::mem_docs::{self, *};
use lsp_server::{
  Connection, ExtractError, Message, Request,
  RequestId, Response,
};
use lsp_types::notification::{
  DidChangeTextDocument, Notification,
};
use lsp_types::request::Completion;
use lsp_types::{
  CompletionItem, CompletionList, OneOf,
  TextDocumentSyncKind,
};
use lsp_types::{
  GotoDefinitionResponse, InitializeParams,
  ServerCapabilities, request::GotoDefinition,
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::error::Error;
use std::mem;
use std::path::PathBuf;
use tracing::{Level, event};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{
  RollingFileAppender, Rotation,
};
use tracing_subscriber::prelude::*;

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
