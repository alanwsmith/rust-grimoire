use std::env;

fn main() {
    println!("HERE");
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
