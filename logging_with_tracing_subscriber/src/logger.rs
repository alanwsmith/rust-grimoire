use crate::logger_custom_format::MiniFormat;
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
  json_dir: Option<PathBuf>,
  json_level: Option<LevelFilter>,
  txt_dir: Option<PathBuf>,
  txt_level: Option<LevelFilter>,
}

impl Logger {
  pub fn new() -> Self {
    Self {
      guard: None,
      stdout: None,
      stderr: None,
      json_dir: None,
      json_level: None,
      txt_dir: None,
      txt_level: None,
    }
  }

  pub fn setup() -> Self {
    Self {
      guard: None,
      stdout: None,
      stderr: None,
      json_dir: None,
      json_level: None,
      txt_dir: None,
      txt_level: None,
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

  pub fn to_json_dir(
    self,
    dir: &PathBuf,
    level: LevelFilter,
  ) -> Self {
    Self {
      json_dir: Some(dir.to_path_buf()),
      json_level: Some(level),
      ..self
    }
  }

  pub fn to_txt_dir(
    self,
    dir: &PathBuf,
    level: LevelFilter,
  ) -> Self {
    Self {
      txt_dir: Some(dir.to_path_buf()),
      txt_level: Some(level),
      ..self
    }
  }

  pub fn init(mut self) -> Option<WorkerGuard> {
    let json_dir_layer =
      match (&self.json_dir, &self.json_level) {
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

    let stdout_layer = match self.stdout {
      Some(level) => {
        let layer = fmt::Layer::default()
          .event_format(MiniFormat)
          .with_writer(std::io::stdout)
          .with_filter(level);
        Some(layer)
      }
      None => None,
    };

    let txt_dir_layer =
      match (&self.txt_dir, &self.txt_level) {
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

          let file_layer_format = MiniFormat;

          // let file_layer_format =
          //   tracing_subscriber::fmt::format()
          //     .without_time()
          //     .with_target(false)
          //     .with_thread_ids(false)
          //     .with_thread_names(false)
          //     .with_ansi(false)
          //     .with_line_number(false)
          //     .with_file(false);

          let layer = fmt::Layer::default()
            .event_format(file_layer_format)
            .with_writer(file_writer)
            .with_filter(*level);
          Some(layer)
        }
        _ => None,
      };

    let subscriber =
      tracing_subscriber::Registry::default()
        .with(json_dir_layer)
        .with(stderr_layer)
        .with(stdout_layer)
        .with(txt_dir_layer);

    tracing::subscriber::set_global_default(subscriber)
      .expect("unable to set global subscriber");

    self.guard
  }
}

impl Default for Logger {
  fn default() -> Self {
    Self::new()
  }
}
