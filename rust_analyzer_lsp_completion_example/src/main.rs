use lsp_server::{
  Connection, ExtractError, Message, Request,
  RequestId, Response,
};
use lsp_types::OneOf;
use lsp_types::{
  GotoDefinitionResponse, InitializeParams,
  ServerCapabilities, request::GotoDefinition,
};
use std::error::Error;
use std::path::PathBuf;
use tracing::{Level, event};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{
  RollingFileAppender, Rotation,
};
use tracing_subscriber::prelude::*;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
  let _logger_guard = init_logger(&PathBuf::from("."));
  event!(Level::INFO, "Starting generic LSP server");

  event!(Level::INFO, "Creating stdio transport");
  let (connection, io_threads) = Connection::stdio();

  event!(Level::INFO, "Defining server capabilities");
  let server_capabilities =
    serde_json::to_value(&ServerCapabilities {
      definition_provider: Some(OneOf::Left(true)),
      ..Default::default()
    })
    .unwrap();

  event!(Level::INFO, "Waiting on initilization");
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

  event!(Level::INFO, "Shutting down gracefully");
  Ok(())
}

fn main_loop(
  connection: Connection,
  params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
  let _params: InitializeParams =
    serde_json::from_value(params).unwrap();

  event!(Level::INFO, "Starting main loop");

  for msg in &connection.receiver {
    eprintln!("got msg: {msg:?}");
    match msg {
      Message::Request(req) => {
        if connection.handle_shutdown(&req)? {
          return Ok(());
        }
        eprintln!("got request: {req:?}");
        match cast::<GotoDefinition>(req) {
          Ok((id, params)) => {
            eprintln!(
              "got gotoDefinition request #{id}: {params:?}"
            );
            let result = Some(
              GotoDefinitionResponse::Array(Vec::new()),
            );
            let result =
              serde_json::to_value(&result).unwrap();
            let resp = Response {
              id,
              result: Some(result),
              error: None,
            };
            connection
              .sender
              .send(Message::Response(resp))?;
            continue;
          }
          Err(err @ ExtractError::JsonError { .. }) => {
            panic!("{err:?}")
          }
          Err(ExtractError::MethodMismatch(req)) => req,
        };
        // ...
      }
      Message::Response(resp) => {
        eprintln!("got response: {resp:?}");
      }
      Message::Notification(not) => {
        eprintln!("got notification: {not:?}");
      }
    }
  }
  Ok(())
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
