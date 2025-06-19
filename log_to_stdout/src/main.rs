use anyhow::Result;
use grimoire_project::logger::Logger;
use tracing::{Level, event, instrument};

fn main() -> Result<()> {
  let _ = Logger::new("DEBUG")?;

  event!(Level::INFO, "In main");
  example_fn_alfa();
  example_fn_bravo();

  Ok(())
}

fn example_fn_alfa() {
  event!(Level::INFO, "In alfa");
}

#[instrument]
fn example_fn_bravo() {
  event!(Level::INFO, "In bravo");
  example_fn_charlie();
}

#[instrument]
fn example_fn_charlie() {
  event!(Level::INFO, "In charlie");
}
