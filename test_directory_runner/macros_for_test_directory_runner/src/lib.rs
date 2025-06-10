use anyhow::Result;
use proc_macro::{TokenStream, TokenTree};
use std::{fs, path::PathBuf};

#[proc_macro]
pub fn test_dir(input: TokenStream) -> TokenStream {
  let tokens: Vec<_> = input.into_iter().collect();
  if let Ok(params) = get_params(tokens) {
    match get_functions(&params) {
      Ok(functions) => functions,
      Err(_) => explode(&params[0]),
    }
  } else {
    explode("failed to get params")
  }
  .parse()
  .unwrap()
}

fn get_payload(path: &PathBuf) -> Result<Vec<String>> {
  let input = fs::read_to_string(path).unwrap();
  let parts = input
    .split("_____")
    .map(|i| i.trim().to_string())
    .collect();
  Ok(parts)
}

fn make_function(path: &PathBuf) -> String {
  let test_name = path.file_stem().unwrap().display();
  let payload = get_payload(path).unwrap();
  format!(
    r#"
#[test]
fn {}() {{
    let left = test_router("{}");
    let right = {};
    assert_eq!(left, right);
}}"#,
    test_name, payload[0], payload[1]
  )
}

fn explode(path: &str) -> String {
  format!(
    r#"
#[test]
fn explode_because_dir_problem() {{
    assert_eq!(
        "ERROR with dir:", 
        {}
    );
}}
    "#,
    path
  )
}

fn get_functions(
  params: &Vec<String>
) -> Result<String> {
  Ok(
    fs::read_dir(&params[0])?
      .filter_map(|f| {
        let path_buf = f.unwrap().path().to_path_buf();
        if let Some(ext) = path_buf.extension() {
          if ext == "customtest" {
            return Some(path_buf);
          }
        }
        None
      })
      .map(|path| make_function(&path))
      .collect::<Vec<String>>()
      .join("\n"),
  )
}

fn get_params(
  tokens: Vec<TokenTree>
) -> Result<Vec<String>> {
  Ok(
    tokens
      .iter()
      .filter_map(|token| {
        if let TokenTree::Literal(literal) = token {
          let mut text = literal.to_string();
          text.pop();
          text.remove(0);
          Some(text)
        } else {
          None
        }
      })
      .collect(),
  )
}
