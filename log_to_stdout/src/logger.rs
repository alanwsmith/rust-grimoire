use anyhow::Result;
use tracing::Event;
use tracing::Subscriber;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::fmt::FormatEvent;
use tracing_subscriber::fmt::FormatFields;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::fmt::time::SystemTime;
use tracing_subscriber::prelude::*;
use tracing_subscriber::registry::LookupSpan;

pub struct Logger {}

impl Logger {
  pub fn new(level: &str) -> Result<Logger> {
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
        .event_format(MiniFormat)
        .with_ansi(false)
        .with_writer(std::io::stdout)
        .with_filter(level_filter);

    let subscriber =
      tracing_subscriber::Registry::default()
        .with(layer);

    tracing::subscriber::set_global_default(
      subscriber,
    )?;

    let logger = Logger {};
    Ok(logger)
  }
}

pub struct MiniFormat;
impl<S, N> FormatEvent<S, N> for MiniFormat
where
  S: Subscriber + for<'a> LookupSpan<'a>,
  N: for<'a> FormatFields<'a> + 'static,
{
  fn format_event(
    &self,
    ctx: &FmtContext<'_, S, N>,
    mut writer: Writer<'_>,
    event: &Event<'_>,
  ) -> Result<(), std::fmt::Error> {
    let meta = event.metadata();
    write!(writer, "{} ", meta.level())?;
    let _ = SystemTime.format_time(&mut writer);
    writeln!(writer)?;
    if let Some(line_number) = meta.line() {
      write!(writer, "Line: {}", line_number,)?;
    }
    if let Some(filename) = meta.file() {
      write!(writer, " - {}", filename)?;
    }
    writeln!(writer)?;
    ctx.format_fields(writer.by_ref(), event)?;
    writeln!(writer)?;
    writeln!(writer)?;
    Ok(())
  }
}
