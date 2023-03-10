// use envvar::var;
use std::env;

// TBD if you need to add envvar crate or not

fn main() {
    let key = "HOME";
    match env::var(key) {
        Ok(val) => println!("{}: {:?}", key, val),
        Err(e) => println!(
            "couldn't interpret {}: {}",
            key, e
        ),
    }
}
