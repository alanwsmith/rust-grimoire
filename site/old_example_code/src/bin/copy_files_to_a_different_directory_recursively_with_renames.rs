use std::path::PathBuf;
use walkdir::{DirEntry, Error, WalkDir};

fn main() {
    do_copy("../test-dir", "../test-dir-2").unwrap();
    println!("Done");
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn do_copy(source_dir: &str, dest_dir: &str) -> Result<(), Error> {
    let walker = WalkDir::new(source_dir).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let p = entry?.path().to_path_buf();
        let sub_path = &p.strip_prefix(source_dir);
        let mut dest_path = PathBuf::from(dest_dir);
        dest_path.push(sub_path.as_ref().unwrap());
        if p.is_dir() {
            // TODO: make the directory if it doesn't exist
            dbg!(dest_path.display().to_string());
        } else {
            // copy or write the parsed file
            if p.extension().unwrap() == "neo" {
                dbg!((p.display().to_string(), dest_path.display().to_string()));
            } else {
                dbg!((p.display().to_string(), dest_path.display().to_string()));
            }
        }
    }
    Ok(())
}
