use anyhow::Result;
use basic_logging_to_a_file::init_logger::init_logger;
use std::path::PathBuf;
use tracing::{Level, event, instrument};

fn main() -> Result<()> {
  let log_dir = PathBuf::from("scratch-output");
  let _guard = init_logger(&log_dir)?;
  event!(Level::INFO, "In main");
  example_fn_alfa();
  example_fn_bravo();
  Ok(())
}

fn example_fn_alfa() {
  event!(Level::INFO, "In alfa");
}

#[instrument]
fn example_fn_bravo() {
  event!(Level::INFO, "In bravo");
  example_fn_charlie();
}

#[instrument]
fn example_fn_charlie() {
  event!(Level::INFO, "In charlie");
}
