use std::process::Command;

fn main() {
    let args = vec!["-l"];
    let response = Command::new("ls")
        .args(args)
        .output()
        .expect("did not work");
    let text = String::from_utf8_lossy(&response.stdout);
    dbg!(text);
}
