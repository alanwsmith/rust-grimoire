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
      let _string = string_at_position(
        params.1.text_document_position,
        global_state,
      );

      let complection_item = CompletionItem::new_simple(
        "ping".to_string(),
        "this is the ping".to_string(),
      );
      let completion_list = CompletionList {
        is_incomplete: true,
        items: vec![complection_item],
      };
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
