use std::fs;
use std::path::PathBuf;

// Get filenames in a single directory
// without recursing or picking up
// directory or hidden file names

fn main() {
    let file_paths = non_hidden_file_paths();
    dbg!(file_paths);
}

fn non_hidden_file_paths() -> Vec<PathBuf> {
    let paths: Vec<PathBuf> = fs::read_dir(".")
        .unwrap()
        .into_iter()
        .map(|p| p.unwrap())
        .filter(|p| {
            p.file_name()
                .to_str()
                .map(|fname| !fname.starts_with("."))
                .unwrap()
        })
        .map(|p| p.path())
        .collect();
    paths
}
