#!/usr/bin/env cargo -q -Zscript

---
[dependencies]
anyhow = "1.0.98"
permissions = "0.5.1"
---

#![allow(unused)]
use anyhow::Result;
use permissions::*;

// Get the permissions for a file

fn main() -> Result<()> {
    dbg!(is_readable("script.rs")?);
    dbg!(is_writable("script.rs")?);
    dbg!(is_executable("script.rs")?);
    dbg!(is_removable("script.rs")?);
    dbg!(is_creatable("script.rs")?);
    Ok(())
}


