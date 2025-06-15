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
use rust_analyzer_lsp_completion_example::mem_docs::{
  self, *,
};
use rust_analyzer_lsp_completion_example::main_loop::*;
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
use rust_analyzer_lsp_completion_example::server_capabilities::*;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
  let _logger_guard = init_logger(&PathBuf::from("."));
  event!(Level::INFO, "Starting generic LSP server");
  let (connection, io_threads) = Connection::stdio();
  let initialization_params = match connection
    .initialize(server_capabilities())
  {
    Ok(it) => it,
    Err(e) => {
      if e.channel_is_disconnected() {
        io_threads.join()?;
      }
      return Err(e.into());
    }
  };
  event!(Level::INFO, "Initilizalized");
  main_loop(connection, initialization_params)?;
  io_threads.join()?;
  event!(Level::INFO, "Shutting down gracefully");
  Ok(())
}

fn cast_notify<N>(
  notif: lsp_server::Notification
) -> Result<
  N::Params,
  ExtractError<lsp_server::Notification>,
>
where
  N: lsp_types::notification::Notification,
  N::Params: serde::de::DeserializeOwned,
{
  notif.extract(N::METHOD)
}

fn cast<R>(
  req: Request
) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
  R: lsp_types::request::Request,
  R::Params: serde::de::DeserializeOwned,
{
  req.extract(R::METHOD)
}

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
