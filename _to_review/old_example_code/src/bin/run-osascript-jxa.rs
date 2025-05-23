use duct::cmd;

fn main() {
    let stdout = cmd!(
        "/usr/bin/osascript",
        "-l",
        "JavaScript",
        "-e",
        r#"Application("iTerm2").windows[0].name()"#
    )
    .read()
    .unwrap();
    println!("{}", stdout);
}
