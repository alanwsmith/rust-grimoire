#![allow(unused)]
use anyhow::Result;
use lsp_server::{
  Connection, ExtractError, Message, Request,
  RequestId, Response,
};
use lsp_types::{
  GotoDefinitionResponse, InitializeParams, OneOf,
  ServerCapabilities, request::GotoDefinition,
};

fn main() -> Result<()> {
  eprintln!("Starting the LSP formatter");
  let (connection, io_threads) = Connection::stdio();

  let server_capabilities =
    serde_json::to_value(&ServerCapabilities {
      definition_provider: Some(OneOf::Left(true)),
      ..Default::default()
    })
    .unwrap();

  let initialization_params =
    match connection.initialize(server_capabilities) {
      Ok(it) => it,
      Err(e) => {
        if e.channel_is_disconnected() {
          io_threads.join()?;
        }
        return Err(e.into());
      }
    };

  main_loop(connection, initialization_params)?;
  io_threads.join()?;
  eprintln!("shutting down server");
  Ok(())
}

fn main_loop(
  connection: Connection,
  params: serde_json::Value,
) -> Result<()> {
  let _params: InitializeParams =
    serde_json::from_value(params).unwrap();
  eprintln!("starting example main loop");
  Ok(())
}
