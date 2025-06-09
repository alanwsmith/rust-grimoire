#!/usr/bin/env cargo -Zscript

---
[dependencies]
clap = "4.5.38"
---

use clap::{Arg, Command};

fn main() {
    let matches = Command::new("runner")
    .arg(Arg::new("thing_to_use")
        .index(1)
        .required(true)
        .help("The thing to use when running the script.")
    )
    .get_matches();

    if let Some(thing_to_use) = matches.get_one::<String>("thing_to_use") {
        println!("Found: {}", thing_to_use);
    }
}

// -- metadata
// -- created: 2025-05-26T11:06:45-04:00
// -- updated: 2025-05-26T11:06:45-04:00
// -- id: 2x/dd/hn/lq
// -- status: draft
