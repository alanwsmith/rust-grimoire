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
