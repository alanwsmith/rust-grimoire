use std::num;

use crate::global_state::GlobalState;
use crate::requests::cast::cast_request;
use lsp_server::{Request, Response};
use lsp_types::{
  SemanticToken, SemanticTokens, SemanticTokensResult,
  request::SemanticTokensFullRequest,
};
use serde::de;
use serde_json::Value;
use tracing::{Level, event};
use unicode_segmentation::UnicodeSegmentation;

pub fn semantic_tokens_full(
  message: Request,
  global_state: &GlobalState,
) -> Response {
  let id = message.id.clone();
  match cast_request::<SemanticTokensFullRequest>(
    message,
  ) {
    Ok(params) => {
      event!(Level::TRACE, "{:?}", &params);
      let uri = params.1.text_document.uri.to_string();
      let initial_text =
        &global_state.mem_docs.get(&uri).unwrap().data;
      let highlights = highlight_tokens(
        &initial_text,
        &id.to_string(),
      );

      Response {
        id,
        result: highlights,
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

fn highlight_tokens(
  input: &str,
  id: &str,
) -> Option<Value> {
  // This is a very basic approach to finding
  // text using windows for find the matches.
  // It's meant to provide output for demonstration
  // purposes. It's unlikely the approach is
  // something you'd want to use in
  // production code.

  let mut numbers = vec![];
  let lines = input.lines();
  for (line_index, line) in lines.enumerate() {
    let characters =
      line.graphemes(true).collect::<Vec<&str>>();
    for (window_index, window) in
      characters.windows(4).enumerate()
    {
      if window == &["a", "l", "f", "a"] {
        let delta_line = line_index as u32;
        let delta_start = window_index as u32;

        numbers.push(delta_line);
        numbers.push(delta_start);
        numbers.push(4 as u32);
        numbers.push(0 as u32);
        numbers.push(2 as u32);

        // let payload = vec![
        //   delta_line,
        //   delta_start,
        //   4 as u32,
        //   0 as u32,
        //   2 as u32,
        // ];
        // numbers.push(payload);

        // data.push(SemanticToken {
        //   delta_line,
        //   delta_start,
        //   length: 4,
        //   token_modifiers_bitset: 0,
        //   token_type: 2,
        // })
      }
    }
  }

  let target = numbers.len() / 5;
  for index in (1..target).rev() {
    let current_line_index = index * 5;
    let previous_line_index = (index - 1) * 5;
    let current_start_index = current_line_index + 1;
    let previous_start_index = previous_line_index + 1;

    numbers[current_line_index] = numbers
      [current_line_index]
      - numbers[previous_line_index];
    // numbers[current_start_index] = numbers
    //   [current_start_index]
    //   - numbers[previous_start_index];

    //numbers[position] = numbers[position
    event!(
      Level::ERROR,
      "\n\n----------------------{:?} - {:?}\n\n--------------------",
      index,
      numbers[index * 5]
    );
  }

  // for (i, w) in numbers
  //   .iter_mut()
  //   .step_by(5)
  //   .skip(1)
  //   .rev()
  //   .enumerate()
  // {
  //   let line_a_index = i * 5;
  //   numbers[line_a_index] = 2;
  //   event!(
  //     Level::ERROR,
  //     "\n\n----------------------{:?} - {:?}\n\n--------------------",
  //     i,
  //     w
  //   );
  // }

  let data = vec![];

  // let data: Vec<_> = numbers
  //   .iter()
  //   .rev()
  //   .enumerate()
  //   .map(|(i, v)| SemanticToken {
  //     delta_line: 0,
  //     delta_start: 1,
  //     length: 4,
  //     token_type: 2,
  //     token_modifiers_bitset: 0,
  //   })
  //   .collect();

  // {
  //   v[0] = numbers[i + 1][0] - v[0];
  //   // let check_index = numbers.len() - 1;
  //   // event!(
  //   //   Level::ERROR,
  //   //   "check_index: {:?}",
  //   //   check_index
  //   // );
  //   // if i < numbers.len() - 1 {
  //   //   event!(
  //   //     Level::ERROR,
  //   //     "ccccccccccccccccccccccccccccccccccc"
  //   //   );
  //   //   // v[0] = numbers[i + 1][0] - v[0];
  //   // }
  //   event!(
  //     Level::ERROR,
  //     "\n\n----------------------{:?} - {:?}\n\n--------------------",
  //     i,
  //     v
  //   );
  // }

  // for line_update in (5..numbers.len()).step_by(5) {
  //   event!(
  //     Level::ERROR,
  //     "\n\n----------------------{:?}\n\n--------------------",
  //     line_update
  //   );
  // }

  Some(
    serde_json::to_value(SemanticTokensResult::Tokens(
      SemanticTokens {
        // TODO: Verify this is the right ID to send back
        result_id: Some(id.to_string()),
        data,
      },
    ))
    .unwrap(),
  )
}
