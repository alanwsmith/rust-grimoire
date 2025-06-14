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
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::registry::Scope;

impl<S, N> FormatEvent<S, N> for MiniFormat
where
  S: Subscriber + for<'a> LookupSpan<'a>,
  N: for<'a> FormatFields<'a> + 'static,
  // T: FormatTime,
{
  fn format_event(
    &self,

    ctx: &FmtContext<'_, S, N>,

    mut writer: Writer<'_>,

    event: &Event<'_>,
  ) -> fmt::Result {
    //#[cfg(feature = "tracing-log")]
    // let normalized_meta = event.normalized_metadata();

    ////#[cfg(feature = "tracing-log")]
    //let meta = normalized_meta
    //  .as_ref()
    //  .unwrap_or_else(|| event.metadata());

    // // #[cfg(not(feature = "tracing-log"))]
    // let meta = event.metadata();

    // if the `Format` struct *also* has an ANSI color configuration,

    // override the writer...the API for configuring ANSI color codes on the

    // `Format` struct is deprecated, but we still need to honor those

    // configurations.

    // if let Some(ansi) = self.ansi {
    //   writer = writer.with_ansi(ansi);
    // }

    //    self.format_timestamp(&mut writer)?;

    // if self.display_level {
    //   let fmt_level = {
    //     #[cfg(feature = "ansi")]
    //     {
    //       FmtLevel::new(
    //         meta.level(),
    //         writer.has_ansi_escapes(),
    //       )
    //     }
    //     #[cfg(not(feature = "ansi"))]
    //     {
    //       FmtLevel::new(meta.level())
    //     }
    //   };
    //   write!(writer, "{} ", fmt_level)?;
    // }

    // if self.display_thread_name {
    //   let current_thread = std::thread::current();
    //   match current_thread.name() {
    //     Some(name) => {
    //       write!(
    //         writer,
    //         "{} ",
    //         FmtThreadName::new(name)
    //       )?;
    //     }
    //     // fall-back to thread id when name is absent and ids are not enabled
    //     None if !self.display_thread_id => {
    //       write!(
    //         writer,
    //         "{:0>2?} ",
    //         current_thread.id()
    //       )?;
    //     }
    //     _ => {}
    //   }
    // }

    // if self.display_thread_id {
    //   write!(
    //     writer,
    //     "{:0>2?} ",
    //     std::thread::current().id()
    //   )?;
    // }

    // let fmt_ctx = {
    //   #[cfg(feature = "ansi")]
    //   {
    //     FmtCtx::new(
    //       ctx,
    //       event.parent(),
    //       writer.has_ansi_escapes(),
    //     )
    //   }
    //   #[cfg(not(feature = "ansi"))]
    //   {
    //     FmtCtx::new(&ctx, event.parent())
    //   }
    // };

    // write!(writer, "{}", fmt_ctx)?;

    // let dimmed = writer.dimmed();

    // let mut needs_space = false;

    // if self.display_target {
    //   write!(
    //     writer,
    //     "{}{}",
    //     dimmed.paint(meta.target()),
    //     dimmed.paint(":")
    //   )?;
    //   needs_space = true;
    // }

    // if self.display_filename {
    //   if let Some(filename) = meta.file() {
    //     if self.display_target {
    //       writer.write_char(' ')?;
    //     }
    //     write!(
    //       writer,
    //       "{}{}",
    //       dimmed.paint(filename),
    //       dimmed.paint(":")
    //     )?;
    //     needs_space = true;
    //   }
    // }

    // if self.display_line_number {
    //   if let Some(line_number) = meta.line() {
    //     write!(
    //       writer,
    //       "{}{}{}{}",
    //       dimmed.prefix(),
    //       line_number,
    //       dimmed.suffix(),
    //       dimmed.paint(":")
    //     )?;
    //     needs_space = true;
    //   }
    // }

    // if needs_space {
    //   writer.write_char(' ')?;
    // }

    // ctx.format_fields(writer.by_ref(), event)?;

    // for span in ctx
    //   .event_scope()
    //   .into_iter()
    //   .flat_map(Scope::from_root)
    // {
    //   let exts = span.extensions();
    //   if let Some(fields) =
    //     exts.get::<FormattedFields<N>>()
    //   {
    //     if !fields.is_empty() {
    //       write!(
    //         writer,
    //         " {}",
    //         dimmed.paint(&fields.fields)
    //       )?;
    //     }
    //   }
    // }

    write!(writer, "HERHEHRHEHRHERHEHRHRHE")?;
    writeln!(writer)
  }
}
