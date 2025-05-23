use std::fs;

fn main() {
    let raw =
        fs::read_to_string("/Users/alan/Desktop/tmp.txt")
            .unwrap();

    let lines: Vec<String> = raw
        .lines()
        .map(|line| line.to_string())
        .collect();

    // Look at this to for triming the strings down:
    //         .map(|line| line.to_string().trim().to_string())

    dbg!(lines);
}
