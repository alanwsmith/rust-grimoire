use std::path::PathBuf;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{
  RollingFileAppender, Rotation,
};
use tracing_subscriber::prelude::*;

pub fn init_logger(log_dir: &PathBuf) -> WorkerGuard {
  let appender = RollingFileAppender::builder()
    .rotation(Rotation::DAILY)
    .filename_suffix("json-lines")
    .max_log_files(2)
    .build(log_dir)
    .expect("Could not build tracing appender");
  let (writer, guard) =
    tracing_appender::non_blocking(appender);
  let layer = tracing_subscriber::fmt::Layer::default()
    .with_ansi(false)
    .with_writer(writer)
    .json();
  let subscriber =
    tracing_subscriber::Registry::default().with(layer);
  tracing::subscriber::set_global_default(subscriber)
    .expect(
      "Could not set tracing subscriber global default",
    );
  guard
}
