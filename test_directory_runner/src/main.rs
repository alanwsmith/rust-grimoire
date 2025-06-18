use serde_json::Value;

fn main() {
  println!("Look to the tests for the example");
}

#[allow(unused)]
fn check_data(input: &str) -> Value {
  serde_json::from_str(&input).unwrap()
}

#[cfg(test)]
mod test {
  use super::*;
  use grimoire_example::test_case::TestCase;
  use rstest::rstest;
  use std::path::PathBuf;

  #[rstest]
  fn for_each_file(
    #[files("test_data/*")] case_dir: PathBuf
  ) {
    let tc = TestCase::new(&case_dir);
    let left = tc.target;
    let right = check_data(&tc.input);
    dbg!(&left);
    assert_eq!(left, right);
  }
}
