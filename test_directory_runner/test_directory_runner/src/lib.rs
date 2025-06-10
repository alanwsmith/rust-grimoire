pub fn widget_test(input: &str) -> bool {
  input == "hello"
}

#[cfg(test)]
mod tests {
  #![allow(unused)]
  use super::*;
  use macros_for_test_directory_runner::test_dir;
  use pretty_assertions::assert_eq;

  fn test_router(input: &str) -> bool {
    widget_test(input)
  }

  test_dir!(
    "test_directory_runner/src/test_cases",
    "widget_test"
  );
}
