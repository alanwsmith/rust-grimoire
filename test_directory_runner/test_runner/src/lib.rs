pub fn widget_test() -> bool {
  true
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use test_runner_macros::test_dir;

  #[allow(unused)]
  fn test_router(input: &str) -> bool {
    widget_test()
  }

  test_dir!("test_runner/src/test_cases");
}
