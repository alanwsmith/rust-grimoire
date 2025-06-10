pub fn widget_testing(input: &str) -> bool {
  input == "hello"
}

#[cfg(test)]
mod tests {
  use super::*;
  use macros_for_test_directory_runner::test_dir;
  use pretty_assertions::assert_eq;

  test_dir!(
    "test_directory_runner/src/test_cases",
    "widget_testing"
  );
}
