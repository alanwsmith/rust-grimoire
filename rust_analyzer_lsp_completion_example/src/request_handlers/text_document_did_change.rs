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

pub fn handle_text_document_did_change(
  mem_docs: &mut MemDocs,
  notif: lsp_server::Notification,
) {
  match cast_notify::<DidChangeTextDocument>(notif) {
    Ok(params) => {
      event!(Level::INFO, "{:?}", params);
      let uri = params.text_document.uri.to_string();
      let version = params.text_document.version;
      let text =
        params.content_changes[0].text.to_string();
      mem_docs
        .insert(&uri, DocumentData::new(version, text));
    }
    Err(e) => (),
  }
}
