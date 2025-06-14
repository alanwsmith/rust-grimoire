use logging_with_tracing_subscriber::logger::Logger;
use std::path::PathBuf;
use tracing::metadata::LevelFilter;
use tracing::{Level, event, instrument};

#[instrument]
fn main() {
  let json_dir = PathBuf::from("test-output/json");
  let txt_dir = PathBuf::from("test-output/txt");
  let _log_guards = Logger::setup()
    .with_stdout(LevelFilter::INFO)
    .with_stderr(LevelFilter::INFO)
    .to_json_dir(&json_dir, LevelFilter::INFO)
    .to_txt_dir(&txt_dir, LevelFilter::INFO)
    .init();
  event!(Level::INFO, "In main");
  alfa();
  bravo();
  println!("process complete.");
}

fn alfa() {
  event!(Level::INFO, "In alfa");
}

#[instrument]
fn bravo() {
  event!(Level::INFO, "In bravo");
  charlie();
}

#[instrument]
fn charlie() {
  event!(Level::INFO, "In charlie");
}
