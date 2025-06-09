#!/usr/bin/env cargo -Zscript

---
[dependencies]
clap = "4.5.38"
---

// Get a single argument from the command
// line with clap

use clap::{Arg, Command};
use std::path::PathBuf;

fn main() {
    let matches = Command::new("runner")
    .arg(Arg::new("thing_to_use")
        .index(1)
        .required(true)
        .value_parser(clap::value_parser!(PathBuf))
        .help("The thing to use when running the script.")
    )
    .get_matches();

    if let Some(thing_to_use) = matches.get_one::<PathBuf>("thing_to_use") {
        println!("Found: {}", thing_to_use.display());
    }
}

// -- metadata 
// -- created: 2025-05-26T13:43:08-04:00
// -- updated: 2025-05-26T13:43:08-04:00
// -- id: 2x/dw/j1/ll
// -- status: done 

