use logging_with_tracing_subscriber::logger::Logger;
use std::path::PathBuf;
use tracing::metadata::LevelFilter;
use tracing::{Level, event, instrument};

fn main() {
  let log_dir = PathBuf::from("test-output");
  let _log_guard = Logger::setup()
    .with_stdout(LevelFilter::INFO)
    .with_stderr(LevelFilter::INFO)
    .with_files(&log_dir, LevelFilter::INFO)
    .init();
  event!(Level::INFO, "IN MAIN");
  alfa();
  bravo();
  println!("Process complete.");
}

fn alfa() {
  event!(Level::INFO, "IN ALFA");
}

#[instrument]
fn bravo() {
  event!(Level::INFO, "IN BRAVO");
  charlie();
}

#[instrument]
fn charlie() {
  event!(Level::INFO, "IN CHARLIE");
}
