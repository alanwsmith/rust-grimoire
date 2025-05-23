use std::fs;
use std::time::UNIX_EPOCH;

fn main() {
    let mod_time = file_epoch_mod_time("/Users/alan/Desktop/here.txt");
    dbg!(mod_time);
}

pub fn file_epoch_mod_time(path: &str) -> u64 {
    fs::metadata(path)
        .unwrap()
        .modified()
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
