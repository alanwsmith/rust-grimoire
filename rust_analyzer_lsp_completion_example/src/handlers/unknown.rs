use tracing::{Level, event};

pub fn unknown() {
  event!(Level::INFO, "unknown message");
}
