use crate::global_state::GlobalState;
use crate::helpers::last_position::last_position;
use crate::requests::cast::cast_request;
use lsp_server::{Request, Response};
use lsp_types::TextEdit;
use lsp_types::request::Formatting;
use std::cmp::max;
use tracing::{Level, event};

pub fn formatting(
  message: Request,
  global_state: &GlobalState,
) -> Response {
  let id = message.id.clone();
  match cast_request::<Formatting>(message) {
    Ok(params) => {
      event!(Level::TRACE, "{:?}", &params);
      let uri = params.1.text_document.uri.to_string();
      let initial_text =
        &global_state.mem_docs.get(&uri).unwrap().data;
      let initial_last_position =
        last_position(&initial_text).unwrap();
      let new_text = formatted_text(initial_text);
      let last_position = max(
        last_position(&new_text).unwrap(),
        initial_last_position,
      );
      let edits = vec![TextEdit {
        new_text,
        range: lsp_types::Range {
          start: lsp_types::Position {
            line: 0,
            character: 0,
          },
          end: lsp_types::Position {
            line: last_position.0,
            character: last_position.1,
          },
        },
      }];
      Response {
        id,
        result: Some(
          serde_json::to_value(&edits).unwrap(),
        ),
        error: None,
      }
    }
    Err(e) => {
      event!(Level::ERROR, "{}", e);
      Response {
        id,
        result: None,
        // TODO: Figure out what error to send back
        error: None,
      }
    }
  }
}

// This is just a basic function to provide a
// working example. It adds a "." at the start
// of any line that doesn't have one.
fn formatted_text(initial_text: &str) -> String {
  initial_text
    .lines()
    .map(|line| {
      if !line.starts_with(".") {
        format!(".{}", line)
      } else {
        line.to_string()
      }
    })
    .collect::<Vec<String>>()
    .join("\n")
}
