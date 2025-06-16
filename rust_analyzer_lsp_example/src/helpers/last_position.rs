use anyhow::{Result, anyhow};
use unicode_segmentation::UnicodeSegmentation;

// Returns the line and character number of the
// last character in the text input.
//
// Uses graphemes to handle unicode stuff.

pub fn last_position(text: &str) -> Result<(u32, u32)> {
  // Add a newline because .lines removes it.
  let check_text = format!("{}\n", text);
  let lines: Vec<&str> = check_text.lines().collect();
  let line_count = lines.len() - 1;
  let last_char = lines
    .iter()
    .last()
    .ok_or(anyhow!("could not get last line"))?
    .graphemes(true)
    .collect::<Vec<&str>>()
    .len();
  Ok((line_count as u32, last_char as u32))
}
