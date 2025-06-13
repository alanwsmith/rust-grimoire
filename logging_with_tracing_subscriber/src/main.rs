#![allow(unused)]
use std::path::PathBuf;
use tracing::{Level, Subscriber, event, instrument};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::{self, Layer};
use tracing_subscriber::layer::Layered;
use tracing_subscriber::prelude::*;

struct Logger {
  guard: WorkerGuard,
}

impl Logger {
  pub fn new() -> Logger {
    let log_dir = PathBuf::from("test-output");
    let log_basename = "example-output.log".to_string();
    // set_up_logging(&log_dir, &log_basename);
    let file_appender =
      tracing_appender::rolling::never(
        log_dir,
        log_basename,
      );
    let (file_writer, guard) =
      tracing_appender::non_blocking(file_appender);
    let file_layer_format =
      tracing_subscriber::fmt::format().json();
    let file_layer = fmt::Layer::default()
      .event_format(file_layer_format)
      .with_writer(file_writer)
      .json();
    let subscriber =
      tracing_subscriber::Registry::default()
        .with(file_layer);
    tracing::subscriber::set_global_default(subscriber)
      .expect("unable to set global subscriber");
    let logger = Logger { guard };
    logger
  }
}

#[instrument]
fn main() {
  let logger_guard = Logger::new();

  // [x] let file_appender = tracing_appender::rolling::never(
  // [x]   log_root,
  // [x]   log_basename,
  // [x] );
  // [x] let (file_writer, _guard) =
  // [x]   tracing_appender::non_blocking(file_appender);
  // [x] let file_layer_format =
  // [x]   tracing_subscriber::fmt::format().json();
  // [x] let file_layer = fmt::Layer::default()
  // [x]   .event_format(file_layer_format)
  // [x]   .with_writer(file_writer)
  // [x]   .json();
  // [] let stdout_format = tracing_subscriber::fmt::format()
  // []   .without_time()
  // []   .with_target(false)
  // []   .with_thread_ids(false)
  // []   .with_thread_names(false)
  // []   .with_ansi(false)
  // []   .with_line_number(false)
  // []   .with_file(false);
  // [] let stdout_layer = fmt::Layer::default()
  // []   .event_format(stdout_format)
  // []   .with_writer(std::io::stdout)
  // []   .with_filter(filter::LevelFilter::INFO);
  // [] let subscriber =
  // []   tracing_subscriber::Registry::default()
  // []     .with(file_layer)
  // []     .with(stdout_layer);
  // [] tracing::subscriber::set_global_default(subscriber)
  // []   .expect("unable to set global subscriber");

  // let file_appender = tracing_appender::rolling::never(
  //   "test-stuff/",
  //   "example-output.log",
  // );
  // let (file_writer, _guard) =
  //   tracing_appender::non_blocking(file_appender);
  // tracing_subscriber::fmt()
  //   .with_writer(file_writer)
  //   .init();

  // tracing_subscriber::registry()
  //   .with(fmt::layer())
  //   .init();

  event!(Level::INFO, "IN MAIN");
  alfa();
  bravo();
  println!("Process complete.");
}

fn set_up_logging(
  log_dir: &PathBuf,
  log_basename: &str,
) {
  let file_appender = tracing_appender::rolling::never(
    log_dir,
    log_basename,
  );
  let (file_writer, _guard) =
    tracing_appender::non_blocking(file_appender);
  let file_layer_format =
    tracing_subscriber::fmt::format().json();
  let file_layer = fmt::Layer::default()
    .event_format(file_layer_format)
    .with_writer(file_writer)
    .json();
  let subscriber =
    tracing_subscriber::Registry::default()
      .with(file_layer);
  tracing::subscriber::set_global_default(subscriber)
    .expect("unable to set global subscriber");
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
