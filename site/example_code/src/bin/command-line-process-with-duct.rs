use duct::cmd;

fn main() {
    let stdout =
        cmd!("echo", "hi").read().unwrap();
    println!("{}", stdout);
}
