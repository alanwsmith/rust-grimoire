fn main() {
  println!("Look to the tests for the example");
}

#[allow(unused)]
fn check_data(input: &str) -> String {
  input.to_string()
}

#[cfg(test)]
mod test {
  use super::*;
  use rstest::rstest;
  use std::{fs, path::PathBuf};

  #[rstest]
  fn for_each_file(
    #[files("test_data/*")] path: PathBuf
  ) {
    let input_file = path.join("input.txt");
    let output_file = path.join("output.txt");
    let input = fs::read_to_string(input_file).unwrap();
    let left = fs::read_to_string(output_file).unwrap();
    let right = check_data(&input);
    assert_eq!(left, right);
  }
}
