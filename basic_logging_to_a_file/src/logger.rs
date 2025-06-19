use anyhow::Result;
use std::path::PathBuf;
use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{
  RollingFileAppender, Rotation,
};
use tracing_subscriber::prelude::*;

pub struct Logger {
  _guard: WorkerGuard,
}

impl Logger {
  pub fn new(
    log_dir: &PathBuf,
    level: &str,
  ) -> Result<Logger> {
    let appender = RollingFileAppender::builder()
      .rotation(Rotation::DAILY)
      .filename_suffix("log")
      .max_log_files(2)
      .build(log_dir)?;

    let (writer, _guard) =
      tracing_appender::non_blocking(appender);

    let level_filter = match level {
      "OFF" => LevelFilter::OFF,
      "TRACE" => LevelFilter::TRACE,
      "DEBUG" => LevelFilter::DEBUG,
      "INFO" => LevelFilter::INFO,
      "WARN" => LevelFilter::WARN,
      "ERROR" => LevelFilter::ERROR,
      _ => LevelFilter::INFO,
    };

    let layer =
      tracing_subscriber::fmt::Layer::default()
        .with_ansi(false)
        .with_writer(writer)
        .json()
        .with_filter(level_filter);

    let subscriber =
      tracing_subscriber::Registry::default()
        .with(layer);

    tracing::subscriber::set_global_default(
      subscriber,
    )?;

    let logger = Logger { _guard };
    Ok(logger)
  }
}
