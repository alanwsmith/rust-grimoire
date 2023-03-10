use std::fs;

fn main() {
    let data = fs::read_to_string(
        "/Users/alan/Desktop/tmp.txt",
    );
    match data {
        Ok(value) => println!("{}", value),
        Err(e) => println!("{}", e),
    }
}
