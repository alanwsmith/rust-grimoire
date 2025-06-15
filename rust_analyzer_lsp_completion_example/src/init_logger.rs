#![allow(unused)]
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

pub fn init_logger(log_dir: &PathBuf) -> WorkerGuard {
  let appender = RollingFileAppender::builder()
    .rotation(Rotation::DAILY)
    .filename_suffix("log")
    .max_log_files(2)
    .build(log_dir)
    .expect("Could not build tracing appender");
  let (writer, guard) =
    tracing_appender::non_blocking(appender);
  let layer = tracing_subscriber::fmt::Layer::default()
    .with_ansi(false)
    .with_writer(writer)
    .pretty();
  let subscriber =
    tracing_subscriber::Registry::default().with(layer);
  tracing::subscriber::set_global_default(subscriber)
    .expect(
      "Could not set tracing subscriber global default",
    );
  guard
}
