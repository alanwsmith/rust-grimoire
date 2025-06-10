pub fn widget_test(input: &str) -> bool {
  input == "hello"
}

#[cfg(test)]
mod tests {
  #![allow(unused)]
  use super::*;
  use pretty_assertions::assert_eq;
  use test_runner_macros::test_dir;

  fn test_router(input: &str) -> bool {
    widget_test(input)
  }

  test_dir!("test_runner/src/test_cases");
}
