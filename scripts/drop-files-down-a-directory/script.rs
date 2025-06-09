#!/usr/bin/env cargo -q -Zscript

---
[dependencies]
anyhow = "1.0.98"
---

use anyhow::Result;
use std::fs;
use std::path::PathBuf;

// Gets a list of files in a directory with 
// a specific extension, makes a directory
// with the same file_stem (aka name without
// the extension) and then moves the file
// into the newly created directory reaming
// it to `new_name` in the process.


fn main() -> Result<()> {
    let new_name = "snippet.html";
    let dir = PathBuf::from("/Users/alan/workshop/bitty-js/build-input/example-html");
    let files = get_files(&dir, "html")?;
    files.iter().for_each(|f| {
        let stem = f.file_stem().unwrap().display().to_string();
        let parent = f.parent().unwrap();
        let new_dir = parent.join(&stem);
        let move_path = new_dir.join(new_name);
        std::fs::create_dir_all(new_dir);
        std::fs::rename(f, move_path);



        // dbg!(parent);
        // dbg!(stem);
        // dbg!(new_dir);
        // dbg!(move_path);

    ()
    }
    );
    Ok(())
}

pub fn get_files(dir: &PathBuf, extension: &str) -> Result<Vec<PathBuf>> {
Ok(
    fs::read_dir(dir)
        .unwrap()
        .into_iter()
        .filter(|p| {
            if p.as_ref().unwrap().path().is_file() {
                true
            } else {
                false
            }
        })
        .filter(|p| {
          match p.as_ref().unwrap().path().extension() {
            Some(ext) => ext == extension,
            None => false
          }
        })
        .filter_map(|p| match p.as_ref().unwrap().path().strip_prefix(".") {
            Ok(_) => None,
            Err(_) => Some(p.as_ref().unwrap().path()),
        })
        .collect()
    )
}
