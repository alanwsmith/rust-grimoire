#![allow(unused)]
use lsp_server::{
  Connection, ExtractError, Message, Request,
  RequestId, Response,
};
use lsp_types::request::Completion;
use lsp_types::{
  CompletionItem, CompletionList, OneOf,
};
use lsp_types::{
  GotoDefinitionResponse, InitializeParams,
  ServerCapabilities, request::GotoDefinition,
};
use serde::de::DeserializeOwned;
use serde_json::Value;
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

  event!(Level::INFO, "Waiting on initilization");
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

  main_loop(connection, initialization_params)?;
  io_threads.join()?;

  event!(Level::INFO, "Shutting down gracefully");
  Ok(())
}

fn server_capabilities() -> Value {
  event!(Level::INFO, "Defining server capabilities");
  serde_json::to_value(&ServerCapabilities {
    completion_provider: Some(
      lsp_types::CompletionOptions {
        ..Default::default()
      },
    ),
    ..Default::default()
  })
  .expect("Could not set up server capabilities")
}

// match cast::<GotoDefinition>(req) {
//   Ok((id, params)) => {
//     event!(
//       Level::INFO,
//       "Request ID #{id} - Params {params:?}"
//     );
//     let result = Some(
//       GotoDefinitionResponse::Array(Vec::new()),
//     );
//     let result =
//       serde_json::to_value(&result).unwrap();
//     let resp = Response {
//       id,
//       result: Some(result),
//       error: None,
//     };
//     connection
//       .sender
//       .send(Message::Response(resp))?;
//     continue;
//   }
//   Err(err @ ExtractError::JsonError { .. }) => {
//     panic!("{err:?}")
//   }
//   Err(ExtractError::MethodMismatch(req)) => req,
// };

fn handle_completion(
  req: &Request
) -> Option<Response> {
  event!(Level::INFO, "handling completion");
  let complection_item = CompletionItem::new_simple(
    "ping".to_string(),
    "this is the ping".to_string(),
  );
  let completion_list = CompletionList {
    is_incomplete: false,
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

fn main_loop(
  connection: Connection,
  params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
  event!(Level::INFO, "Starting main loop");

  let _params: InitializeParams =
    serde_json::from_value(params).unwrap();

  for msg in &connection.receiver {
    event!(Level::INFO, "Got message: {msg:?}");

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
            event!(Level::INFO, "--- SENDING ---");
            connection
              .sender
              .send(Message::Response(r))?;
            continue;
          }
          None => (),
        }

        // let x = match req.extract()

        // event!(
        //   Level::INFO,
        //   "method: {0:?}",
        //   req.extract::<Result<
        //     RequestId,
        //     lsp_types::request::Request::Params,
        //   >>(
        //     // serde::de::DeserializeOwned
        //     lsp_types::request::Request::METHOD
        //   )
        // );

        //dbg!(req.method);

        // event!(Level::INFO, "Got request: {req:?}");
        // match req {
        // }

        // match cast::<Completion>(req) {
        //   Ok((id, params)) => {
        //     event!(
        //       Level::INFO,
        //       "Completion request ID #{id} - Params {params:?}"
        //     );
        //     continue;
        //   }
        //   Err(err @ ExtractError::JsonError { .. }) => {
        //     panic!("{err:?}")
        //   }
        //   Err(ExtractError::MethodMismatch(req)) => req,
        // };

        // match cast::<GotoDefinition>(req) {
        //   Ok((id, params)) => {
        //     event!(
        //       Level::INFO,
        //       "Request ID #{id} - Params {params:?}"
        //     );
        //     let result = Some(
        //       GotoDefinitionResponse::Array(Vec::new()),
        //     );
        //     let result =
        //       serde_json::to_value(&result).unwrap();
        //     let resp = Response {
        //       id,
        //       result: Some(result),
        //       error: None,
        //     };
        //     connection
        //       .sender
        //       .send(Message::Response(resp))?;
        //     continue;
        //   }
        //   Err(err @ ExtractError::JsonError { .. }) => {
        //     panic!("{err:?}")
        //   }
        //   Err(ExtractError::MethodMismatch(req)) => req,
        // };

        // ...
      }

      Message::Response(resp) => {
        event!(Level::INFO, "Got response: {resp:?}");
      }

      Message::Notification(notif) => {
        event!(
          Level::INFO,
          "Got notification: {notif:?}"
        );
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
