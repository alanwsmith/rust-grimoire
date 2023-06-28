use svix_ksuid::*;
use time::macros::datetime;

fn main() {
    let raw_ksuid = Ksuid::new(Some(datetime!(2017-11-16 12:12:01 UTC)), None);
    let ksuid_string = raw_ksuid.to_string();
    println!("{}", ksuid_string);
}
