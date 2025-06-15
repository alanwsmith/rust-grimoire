use crate::requests::cast::cast_request;
use lsp_server::{Request, Response};
use lsp_types::{
  CompletionItem, CompletionList, request::Completion,
};
use tracing::{Level, event};

use crate::global_state::GlobalState;

pub fn completion(
  message: Request,
  _global_state: &GlobalState,
) -> Option<Response> {
  match cast_request::<Completion>(message) {
    _ => (),
  }
  None

  // event!(
  //   Level::TRACE,
  //   "Handle Complestion: {:?}",
  //   &message
  // );

  // let complection_item = CompletionItem::new_simple(
  //   "ping".to_string(),
  //   "this is the ping".to_string(),
  // );
  // let completion_list = CompletionList {
  //   is_incomplete: true,
  //   items: vec![complection_item],
  // };
  // Some(Response {
  //   id: message.id.clone(),
  //   result: Some(
  //     serde_json::to_value(completion_list).unwrap(),
  //   ),
  //   error: None,
  // })
}
