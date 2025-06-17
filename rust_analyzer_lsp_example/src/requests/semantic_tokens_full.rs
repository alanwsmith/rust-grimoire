use crate::global_state::GlobalState;
use crate::requests::cast::cast_request;
use lsp_server::{Request, Response};
use lsp_types::{
  CompletionItem, CompletionList, SemanticToken,
  SemanticTokens, SemanticTokensResult,
  request::SemanticTokensFullRequest,
};
use tracing::{Level, event};

pub fn semantic_tokens_full(
  message: Request,
  _global_state: &GlobalState,
) -> Response {
  let id = message.id.clone();
  match cast_request::<SemanticTokensFullRequest>(
    message,
  ) {
    Ok(params) => {
      event!(Level::TRACE, "{:?}", &params);

      let response =
        SemanticTokensResult::Tokens(SemanticTokens {
          // TODO: Verify this is the right ID to send back
          result_id: Some(id.to_string()),
          data: vec![SemanticToken {
            delta_line: 0,
            delta_start: 1,
            length: 4,
            token_modifiers_bitset: 0,
            token_type: 2,
          }],
        });

      Response {
        id,
        result: Some(
          serde_json::to_value(response).unwrap(),
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

// This is just a simple filter to provide
// a working example. It sends back any of
// the `word_list` that matches the current
// input
fn filter_words(input: &str) -> CompletionList {
  let word_list = vec![
    "-- test", "al", "alfa", "bravo", "braavo",
    "charlie",
  ];

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

  CompletionList {
    is_incomplete: filtered.len() > 0,
    items: filtered,
  }
}
