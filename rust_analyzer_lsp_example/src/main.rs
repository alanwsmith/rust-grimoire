use lsp_server::Connection;
use rust_analyzer_lsp_example::init_logger::*;
use rust_analyzer_lsp_example::main_loop::*;
use rust_analyzer_lsp_example::server_capabilities::*;
use std::error::Error;
use std::path::PathBuf;
use tracing::{Level, event};

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
  let _logger_guard = init_logger(&PathBuf::from("."));
  event!(Level::DEBUG, "Starting generic LSP server");
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
  main_loop(connection, initialization_params)?;
  io_threads.join()?;
  event!(Level::DEBUG, "Shutting down gracefully");
  Ok(())
}
