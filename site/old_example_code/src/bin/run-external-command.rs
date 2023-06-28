use duct::cmd;

fn main() {
    let stdout =
        cmd!("echo", "hi").read().unwrap();
    println!("{}", stdout);
}

// There's also this, but it seems
// harder to use:
// use std::process::Command;

// NOTE: Could get some commands to work
// with `Command::new` but couldn't get
// osascript to work with:
// /usr/bin/osascript - "-l",
// "JavaScript", "-e",
// r#"'Application("Google Chro etc.....

// Ended up using run_cmd
// for that: https://docs.rs/cmd_lib/latest/cmd_lib/macro.run_cmd.html

// fn main() {
//     let response =
// Command::new("/usr/bin/osascript")
//         .args(
//             ["-l",
//         "JavaScript",
//         "-e",
//         r#"'Application("Google
// Chrome").windows[0].bounds = { "x": 20, "y":
// 20, "width": 400, "height": 400 }'"#
//             ])
//                 .status()
//                 .expect("HERE");
//     // if response.status.success() {
//     //     let output = String::from_utf8(
//     //         response.stdout,
//     //     )
//     //     .unwrap();
//     //     println!("{}", output);
//     // } else {
//     //     dbg!(response);
//     // }
// }
