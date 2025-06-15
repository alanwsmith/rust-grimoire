use crate::handlers::completion::completion;
use crate::handlers::text_document_did_change::text_document_did_change;
use crate::mem_docs::MemDocs;
use lsp_server::{Connection, Message};
use lsp_types::InitializeParams;
use std::error::Error;
use tracing::{Level, event};

pub fn main_loop(
  connection: Connection,
  params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
  event!(Level::INFO, "Starting main loop");
  let mut mem_docs = MemDocs::default();

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
          "textDocument/completion" => completion(&req),
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
            text_document_did_change(
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
