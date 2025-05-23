use std::fs;

// NOTE: This works, but because they
// are &str they can't be passed around
// easily.

fn main() {
    let file_path = file!();
    let raw = fs::read_to_string(file_path).unwrap();
    let lines: Vec<&str> = raw.lines().collect();
    dbg!(lines);
}
