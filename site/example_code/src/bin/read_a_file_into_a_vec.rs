use std::fs;

fn main() {
    let raw =
        fs::read_to_string("~/Desktop/tmp.txt")
            .unwrap();

    let lines: Vec<&str> = raw.lines().collect();

    dbg!(lines);
}
