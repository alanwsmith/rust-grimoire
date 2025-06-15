use crate::global_state::GlobalState;
use anyhow::{Result, anyhow};
use lsp_types::TextDocumentPositionParams;
use tracing::{Level, event};
use unicode_segmentation::UnicodeSegmentation;

pub fn string_at_position(
  text_document_position_params: TextDocumentPositionParams,
  global_state: &GlobalState,
) -> Result<String> {
  let uri = text_document_position_params
    .text_document
    .uri
    .to_string();
  let line = text_document_position_params.position.line
    as usize;
  let character = text_document_position_params
    .position
    .character as usize;

  let doc = &global_state
    .mem_docs
    .get(&uri)
    .ok_or(anyhow!("could not get do"))?
    .data;

  let lines: Vec<&str> = doc.lines().collect();
  let characters =
    UnicodeSegmentation::graphemes(lines[line], true)
      .take(character)
      .collect::<Vec<&str>>();

  event!(
    Level::DEBUG,
    "\n----------------------------\n\n{} - {} - {}\n\n{}\n\n{}\n\n---------------------------",
    line,
    character,
    uri.to_string(),
    lines[line],
    characters.join("")
  );

  Ok("x".to_string())
}
