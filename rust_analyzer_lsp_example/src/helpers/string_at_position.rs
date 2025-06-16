use crate::global_state::GlobalState;
use anyhow::{Result, anyhow};
use lsp_types::TextDocumentPositionParams;
use unicode_segmentation::UnicodeSegmentation;

// starts and the given position and finds
// all preceding characters until a space.

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
  let characters: Vec<&str> =
    UnicodeSegmentation::graphemes(lines[line], true)
      .take(character)
      .collect::<Vec<&str>>()
      .iter()
      .rev()
      .take_while(|c| **c != " ")
      .map(|c| *c)
      .collect::<Vec<&str>>()
      .iter()
      .rev()
      .map(|c| *c)
      .collect();
  Ok(characters.join("").to_string())
}
