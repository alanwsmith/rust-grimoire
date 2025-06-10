use anyhow::Result;
use proc_macro::{TokenStream, TokenTree};
use std::{fs, path::PathBuf};

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

fn get_functions(path: &str) -> Result<String> {
  Ok(
    fs::read_dir(path)?
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

#[proc_macro]
pub fn test_dir(input: TokenStream) -> TokenStream {
  let tokens: Vec<_> = input.into_iter().collect();
  let path = get_path(tokens);
  match get_functions(&path) {
    Ok(functions) => functions,
    Err(_) => explode(&path),
  }
  .parse()
  .unwrap()
}

fn get_path(tokens: Vec<TokenTree>) -> String {
  match tokens.as_slice() {
    [TokenTree::Literal(lit)] => {
      unwrap_string_literal(&lit)
    }
    _ => panic!("Must be single, non-empty string"),
  }
}

fn unwrap_string_literal(
  lit: &proc_macro::Literal
) -> String {
  let mut repr = lit.to_string();
  if !repr.starts_with('"') || !repr.ends_with('"') {
    panic!("Must be single, non-empty string")
  }
  repr.remove(0);
  repr.pop();
  repr
}
