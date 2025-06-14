pub struct MiniFormat;
use std::fmt;
use tracing::Event;
use tracing::Subscriber;
use tracing_subscriber::fmt::FmtContext;
use tracing_subscriber::fmt::FormatEvent;
use tracing_subscriber::fmt::FormatFields;
use tracing_subscriber::fmt::FormattedFields;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::fmt::time::SystemTime;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::registry::Scope;

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
  ) -> fmt::Result {
    let meta = event.metadata();
    write!(writer, "{} ", meta.level())?;
    let _ = SystemTime.format_time(&mut writer);
    if let Some(line_number) = meta.line() {
      write!(writer, " Line: {}", line_number,)?;
    }
    if let Some(filename) = meta.file() {
      write!(writer, " {}", filename)?;
    }
    writeln!(writer)?;
    ctx.format_fields(writer.by_ref(), event)?;
    for span in ctx
      .event_scope()
      .into_iter()
      .flat_map(Scope::from_root)
    {
      let exts = span.extensions();
      if let Some(fields) =
        exts.get::<FormattedFields<N>>()
      {
        if !fields.is_empty() {
          write!(writer, " {}", &fields.fields)?;
        }
      }
    }
    writeln!(writer)
  }
}
