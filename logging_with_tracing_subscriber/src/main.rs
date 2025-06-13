#![allow(unused)]
use anyhow::Result;
use std::path::PathBuf;
use tracing::metadata::LevelFilter;
use tracing::{Level, Subscriber, event, instrument};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::{self, Layer};
use tracing_subscriber::layer::Layered;
use tracing_subscriber::{Registry, prelude::*};

#[derive(Debug)]
pub struct Logger {
  pub guard: Option<WorkerGuard>,
  stdout: Option<LevelFilter>,
  stderr: Option<LevelFilter>,
  file_dir: Option<PathBuf>,
  file_level: Option<LevelFilter>,
}

// let stdout_format = tracing_subscriber::fmt::format()
//     .without_time()
//     .with_target(false)
//     .with_thread_ids(false)
//     .with_thread_names(false)
//     .with_ansi(false)
//     .with_line_number(false)
//     .with_file(false);
// let stdout_layer = fmt::Layer::default()
//     .event_format(stdout_format)
//     .with_writer(std::io::stdout)
//     .with_filter(filter::LevelFilter::INFO);

impl Logger {
  pub fn new() -> Self {
    Self {
      guard: None,
      stdout: None,
      stderr: None,
      file_dir: None,
      file_level: None,
    }
    //    Self { guard: None }
    //let log_dir = PathBuf::from("test-output");
    //let log_basename = "example-output.log".to_string();
    //// set_up_logging(&log_dir, &log_basename);
    //let file_appender =
    //  tracing_appender::rolling::never(
    //    log_dir,
    //    log_basename,
    //  );
    //let (file_writer, guard) =
    //  tracing_appender::non_blocking(file_appender);
    //let file_layer_format =
    //  tracing_subscriber::fmt::format().json();
    //let file_layer = fmt::Layer::default()
    //  .event_format(file_layer_format)
    //  .with_writer(file_writer)
    //  .json();
    //let subscriber =
    //  tracing_subscriber::Registry::default()
    //    .with(file_layer);
    //tracing::subscriber::set_global_default(subscriber)
    //  .expect("unable to set global subscriber");
    //// guard
    //Logger { guard }
    ////let logger = Logger { guard };
    ////logger
  }

  pub fn setup() -> Self {
    Self {
      guard: None,
      stdout: None,
      stderr: None,
      file_dir: None,
      file_level: None,
    }
  }

  pub fn with_stdout(
    self,
    level: LevelFilter,
  ) -> Self {
    Self {
      stdout: Some(level),
      ..self
    }
  }

  pub fn with_stderr(
    self,
    level: LevelFilter,
  ) -> Self {
    Self {
      stderr: Some(level),
      ..self
    }
  }

  pub fn with_files(
    self,
    dir: &PathBuf,
    level: LevelFilter,
  ) -> Self {
    Self {
      file_dir: Some(dir.to_path_buf()),
      file_level: Some(level),
      ..self
    }
  }

  pub fn init(mut self) -> Self {
    let stdout_layer = match self.stdout {
      Some(level) => {
        let format = tracing_subscriber::fmt::format()
          .without_time()
          .with_target(false)
          .with_thread_ids(false)
          .with_thread_names(false)
          .with_ansi(false)
          .with_line_number(false)
          .with_file(false);
        let layer = fmt::Layer::default()
          .event_format(format)
          .with_writer(std::io::stdout)
          .with_filter(level);
        Some(layer)
      }
      None => None,
    };

    let stderr_layer = match self.stderr {
      Some(level) => {
        let format = tracing_subscriber::fmt::format()
          .without_time()
          .with_target(false)
          .with_thread_ids(false)
          .with_thread_names(false)
          .with_ansi(false)
          .with_line_number(false)
          .with_file(false);
        let layer = fmt::Layer::default()
          .event_format(format)
          .with_writer(std::io::stderr)
          .with_filter(level);
        Some(layer)
      }
      None => None,
    };

    let file_layer =
      match (&self.file_dir, &self.file_level) {
        (Some(dir), Some(level)) => {
          let file_appender =
            tracing_appender::rolling::never(
              dir, "log.log",
            );
          let (file_writer, log_guard) =
            tracing_appender::non_blocking(
              file_appender,
            );
          self.guard = Some(log_guard);
          let file_layer_format =
            tracing_subscriber::fmt::format().json();
          let layer = fmt::Layer::default()
            .event_format(file_layer_format)
            .with_writer(file_writer)
            .json();
          Some(layer)
        }
        _ => None,
      };

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

    let subscriber =
      tracing_subscriber::Registry::default()
        .with(stdout_layer)
        .with(file_layer)
        .with(stderr_layer);

    tracing::subscriber::set_global_default(subscriber)
      .expect("unable to set global subscriber");

    // let stdout_layer = if self.stdout {
    //   let format = tracing_subscriber::fmt::format()
    //     .without_time()
    //     .with_target(false)
    //     .with_thread_ids(false)
    //     .with_thread_names(false)
    //     .with_ansi(false)
    //     .with_line_number(false)
    //     .with_file(false);
    //   let layer = fmt::Layer::default()
    //     .event_format(format)
    //     .with_writer(std::io::stdout)
    //     .with_filter(LevelFilter::INFO);
    //   Some(layer)
    // } else {
    //   None
    // };

    // let stderr_layer = if self.stderr {
    //   let format = tracing_subscriber::fmt::format()
    //     .without_time()
    //     .with_target(false)
    //     .with_thread_ids(false)
    //     .with_thread_names(false)
    //     .with_ansi(false)
    //     .with_line_number(false)
    //     .with_file(false);
    //   let layer = fmt::Layer::default()
    //     .event_format(format)
    //     .with_writer(std::io::stdout)
    //     .with_filter(LevelFilter::INFO);
    //   Some(layer)
    // } else {
    //   None
    // };

    // let stdout_format =
    //   tracing_subscriber::fmt::format()
    //     .without_time()
    //     .with_target(false)
    //     .with_thread_ids(false)
    //     .with_thread_names(false)
    //     .with_ansi(false)
    //     .with_line_number(false)
    //     .with_file(false);
    // let stdout_layer = fmt::Layer::default();
    //     let stdout_layer = fmt::Layer::default();
    // .event_format(stdout_format)
    // .with_writer(std::io::stdout)
    // .with_filter(LevelFilter::INFO);

    // let subscriber =
    //   tracing_subscriber::Registry::default();
    // let subscriber = subscriber.with(stdout_layer);

    // let subscriber = AssertionLayer
    //   .with_subscriber(Registry::default());

    // let subscriber =
    //   tracing_subscriber::Registry::default();
    // let subscriber = if self.stdout {
    //   subscriber.with(stdout_layer)
    // } else {
    //   subscriber
    // };

    Self { ..self }
  }
}

impl Default for Logger {
  fn default() -> Self {
    Self::new()
  }
}

fn main() {
  let log_dir = PathBuf::from("test-output");
  let logger_guard = Logger::setup()
    .with_stdout(LevelFilter::INFO)
    .with_stderr(LevelFilter::INFO)
    .with_files(&log_dir, LevelFilter::INFO)
    .init();

  // let logger_guard = Logger::setup().init();
  event!(Level::INFO, "IN MAIN");

  //dbg!(logger_guard);

  //  let log_guard = start_logging(false, false, None);

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

  // alfa();
  // bravo();
  println!("Process complete.");
}

// fn set_up_logging(
//   log_dir: &PathBuf,
//   log_basename: &str,
// ) {
//   let file_appender = tracing_appender::rolling::never(
//     log_dir,
//     log_basename,
//   );
//   let (file_writer, _guard) =
//     tracing_appender::non_blocking(file_appender);
//   let file_layer_format =
//     tracing_subscriber::fmt::format().json();
//   let file_layer = fmt::Layer::default()
//     .event_format(file_layer_format)
//     .with_writer(file_writer)
//     .json();
//   let subscriber =
//     tracing_subscriber::Registry::default()
//       .with(file_layer);
//   tracing::subscriber::set_global_default(subscriber)
//     .expect("unable to set global subscriber");
// }

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

// fn start_logging(
//   stdout: bool,
//   stderr: bool,
//   dir: Option<PathBuf>,
// ) -> Result<WorkerGuard> {
//   let log_dir = PathBuf::from("test-output");
//   let log_basename = "example-output.log".to_string();
//   // set_up_logging(&log_dir, &log_basename);
//   let file_appender = tracing_appender::rolling::never(
//     log_dir,
//     log_basename,
//   );
//   let (file_writer, guard) =
//     tracing_appender::non_blocking(file_appender);
//   let file_layer_format =
//     tracing_subscriber::fmt::format().json();
//   let file_layer = fmt::Layer::default()
//     .event_format(file_layer_format)
//     .with_writer(file_writer)
//     .json();
//   let subscriber =
//     tracing_subscriber::Registry::default()
//       .with(file_layer);
//   tracing::subscriber::set_global_default(subscriber)
//     .expect("unable to set global subscriber");
//   Ok(guard)
// }
