use serde_json::Value;
use std::fs;
use std::path::PathBuf;

pub struct TestCase {
  pub input: String,
  pub target: Value,
  pub remainder: String,
}

impl TestCase {
  pub fn new(case_dir: &PathBuf) -> TestCase {
    let input_path = case_dir.join("input.txt");
    let target_path = case_dir.join("target.json");
    let remainder_path = case_dir.join("remainder.txt");
    let input = fs::read_to_string(input_path)
      .unwrap()
      .trim()
      .to_string();
    let target_string =
      fs::read_to_string(target_path).unwrap();
    let target =
      serde_json::from_str(&target_string).unwrap();
    let remainder = fs::read_to_string(remainder_path)
      .unwrap()
      .trim()
      .to_string();
    TestCase {
      input,
      target,
      remainder,
    }
  }
}
