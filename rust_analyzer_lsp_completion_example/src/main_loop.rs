#![allow(unused)]
use crate::request_handlers::completion::handle_completion;
use crate::mem_docs::{self, *};
use crate::notification_handlers::text_document_did_change::*;
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

pub fn main_loop(
  connection: Connection,
  params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
  let mut mem_docs = mem_docs::MemDocs::default();
  event!(Level::INFO, "Starting main loop");

  let _params: InitializeParams =
    serde_json::from_value(params).unwrap();

  for msg in &connection.receiver {
    match msg {
      Message::Request(req) => {
        if connection.handle_shutdown(&req)? {
          return Ok(());
        }

        event!(Level::INFO, "Got request: {req:?}");
        let response = match req.method.as_str() {
          "textDocument/completion" => {
            handle_completion(&req)
          }
          _ => {
            event!(
              Level::INFO,
              "Unhandled request type"
            );
            None
          }
        };

        match response {
          Some(r) => {
            connection
              .sender
              .send(Message::Response(r))?;
            continue;
          }
          None => (),
        }
      }

      Message::Response(resp) => {
        event!(Level::INFO, "Got response: {resp:?}");
      }

      Message::Notification(notif) => {
        match notif.method.as_str() {
          "textDocument/didChange" => {
            handle_text_document_did_change(
              &mut mem_docs,
              notif,
            )
          }
          _ => {
            event!(
              Level::INFO,
              "{}",
              notif.method.as_str()
            );
          }
        }
      }
    }
  }
  Ok(())
}
