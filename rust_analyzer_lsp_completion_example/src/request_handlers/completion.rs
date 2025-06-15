#![allow(unused)]
use crate::mem_docs::*;
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

pub fn handle_completion(
  req: &Request
) -> Option<Response> {
  event!(Level::INFO, "handling completion");
  let complection_item = CompletionItem::new_simple(
    "ping".to_string(),
    "this is the ping".to_string(),
  );
  let completion_list = CompletionList {
    is_incomplete: true,
    items: vec![complection_item],
  };
  Some(Response {
    id: req.id.clone(),
    result: Some(
      serde_json::to_value(completion_list).unwrap(),
    ),
    error: None,
  })
}
