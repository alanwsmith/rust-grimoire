use anyhow::Result;
use proc_macro::{TokenStream, TokenTree};
use std::{fs, path::PathBuf};

fn make_function(path: &PathBuf) -> String {
  let test_name = path.file_stem().unwrap().display();
  format!(
    r#"
#[test]
fn {}() {{
    let left = test_router("a");
    let right = true;
    assert_eq!(left, right);
}}"#,
    test_name
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
    Err(_) => "".to_string(),
  }
  .parse()
  .unwrap()

  // if let Ok(dir_list) = fs::read_dir(&path) {
  //   dir_list
  //     .filter_map(|f| {
  //       let path_buf = f.unwrap().path().to_path_buf();
  //       if let Some(ext) = path_buf.extension() {
  //         if ext == "customtest" {
  //           return Some(path_buf);
  //         }
  //       }
  //       None
  //     })
  //     .map(|f| {
  //       let test_name =
  //         f.file_stem().unwrap().display();
  //       format!(
  //         r#"
  // #[test]
  // fn {}() {{
  //   let left = test_router("a");
  //   let right = true;
  //   assert_eq!(left, right);
  // }}"#,
  //         test_name
  //       )
  //     })
  //     .collect::<Vec<String>>()
  //     .join("\n")
  //     .parse()
  //     .unwrap()
  // } else {
  //   format!(
  //     r#"
  // #[test]
  // fn explode_because_could_not_read_dir() {{
  //   assert_eq!("ERROR reading:", "{}");
  // }}"#,
  //     path
  //   )
  //   .parse()
  //   .unwrap()
  // }
}

fn get_path(tokens: Vec<TokenTree>) -> String {
  match tokens.as_slice() {
    [TokenTree::Literal(lit)] => {
      unwrap_string_literal(&lit)
    }
    _ => panic!(
      "This macro only accepts a single, non-empty string argument"
    ),
  }
}

fn unwrap_string_literal(
  lit: &proc_macro::Literal
) -> String {
  let mut repr = lit.to_string();
  if !repr.starts_with('"') || !repr.ends_with('"') {
    panic!(
      "This macro only accepts a single, non-empty string argument"
    )
  }
  repr.remove(0);
  repr.pop();
  repr
}
