use std::path::PathBuf;
use tracing::metadata::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{
  RollingFileAppender, Rotation,
};
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

pub struct Logger {
  pub guard: Option<WorkerGuard>,
  stdout: Option<LevelFilter>,
  stderr: Option<LevelFilter>,
  file_dir: Option<PathBuf>,
  file_level: Option<LevelFilter>,
}

impl Logger {
  pub fn new() -> Self {
    Self {
      guard: None,
      stdout: None,
      stderr: None,
      file_dir: None,
      file_level: None,
    }
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
            RollingFileAppender::builder()
              .rotation(Rotation::DAILY)
              .filename_prefix("log")
              .filename_suffix("log")
              .max_log_files(2)
              .build(dir)
              .expect("could not make file appender");
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
            .json()
            .with_filter(*level);
          Some(layer)
        }
        _ => None,
      };
    let subscriber =
      tracing_subscriber::Registry::default()
        .with(stdout_layer)
        .with(file_layer)
        .with(stderr_layer);
    tracing::subscriber::set_global_default(subscriber)
      .expect("unable to set global subscriber");
    Self { ..self }
  }
}

impl Default for Logger {
  fn default() -> Self {
    Self::new()
  }
}
