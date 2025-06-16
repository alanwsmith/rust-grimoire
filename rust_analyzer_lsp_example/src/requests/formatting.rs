use crate::global_state::GlobalState;
use crate::helpers::last_position::last_position;
use crate::requests::cast::cast_request;
use lsp_server::{Request, Response};
use lsp_types::TextEdit;
use lsp_types::request::Formatting;
use tracing::{Level, event};

pub fn formatting(
  message: Request,
  global_state: &GlobalState,
) -> Response {
  let id = message.id.clone();
  match cast_request::<Formatting>(message) {
    Ok(params) => {
      event!(
        Level::DEBUG,
        "\n\n7777777777777777777\n\n{:?}\n\n77777777777777777777777\n\n",
        &params
      );

      let uri = params.1.text_document.uri.to_string();
      let initial_text =
        &global_state.mem_docs.get(&uri).unwrap().data;
      let new_text = formatted_text(initial_text);
      let last_position =
        last_position(&new_text).unwrap();
      let edits = vec![TextEdit {
        new_text,
        range: lsp_types::Range {
          start: lsp_types::Position {
            line: 0,
            character: 0,
          },
          end: lsp_types::Position {
            line: last_position.0 as u32,
            character: last_position.1 as u32,
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
      event!(Level::ERROR, "\n\n{}", e);
      Response {
        id,
        result: None,
        // TODO: Figure out what error to send back
        error: None,
      }
    }
  }
}

fn formatted_text(initial_text: &str) -> String {
  initial_text.to_string()
}
