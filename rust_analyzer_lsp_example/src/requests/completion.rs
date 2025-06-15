use crate::global_state::GlobalState;
use crate::helpers::string_at_position::string_at_position;
use crate::requests::cast::cast_request;
use lsp_server::{Request, Response};
use lsp_types::{
  CompletionItem, CompletionList, request::Completion,
};
use tracing::{Level, event};

pub fn completion(
  message: Request,
  global_state: &GlobalState,
) -> Response {
  let id = message.id.clone();
  match cast_request::<Completion>(message) {
    Ok(params) => {
      event!(Level::DEBUG, "\n\n{:?}", &params);
      // TODO: Handle this unwrap better
      let current_string = string_at_position(
        params.1.text_document_position,
        global_state,
      )
      .unwrap();
      let completion_list =
        filter_words(&current_string);

      let result = Some(
        serde_json::to_value(completion_list).unwrap(),
      );
      Response {
        id,
        result,
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

fn filter_words(input: &str) -> CompletionList {
  let word_list =
    vec!["al", "alfa", "bravo", "braavo", "charlie"];

  event!(
    Level::TRACE,
    "\n\n==========\n\n{:?}\n\n===========\n\n",
    input
  );

  let filtered: Vec<_> = word_list
    .iter()
    .filter(|w| w.to_lowercase().starts_with(input))
    .map(|w| {
      CompletionItem::new_simple(
        w.to_string(),
        format!("This is the {}", w),
      )
    })
    .collect();

  event!(
    Level::TRACE,
    "\n\n==========\n\n{:?}\n\n===========\n\n",
    filtered
  );

  let completion_list = CompletionList {
    is_incomplete: filtered.len() > 0,
    items: filtered,
  };

  // let complection_item = CompletionItem::new_simple(
  //   "ping".to_string(),
  //   "this is the ping".to_string(),
  // );
  // let completion_list = CompletionList {
  //   is_incomplete: true,
  //   items: vec![complection_item],
  // };

  completion_list
}
