use anyhow::{Result, anyhow};
use itertools::Itertools;
use permissions::is_executable;
use std::process::Command;
use std::{fs::canonicalize, path::PathBuf};
use walkdir::WalkDir;

pub fn run_scripts(dir: &PathBuf) -> Result<()> {
    if !dir.is_dir() {
        return Err(anyhow!("Not a directory at: {}", dir.display()));
    }
    let walker = WalkDir::new(dir).into_iter();
    let files: Vec<_> = walker
        .filter_map(|entry| match entry {
            Ok(e) => Some(e.path().to_path_buf()),
            Err(_) => None,
        })
        .filter(|pb| pb.is_file())
        .filter(|pb| {
            pb.components()
                .find(|part| part.as_os_str().to_string_lossy().starts_with("."))
                .is_none()
        })
        .sorted()
        .collect();
    for file in files {
        if is_executable(&file)? {
            let name = file.file_name().ok_or(anyhow!("Cound not get file name"))?;
            let parent = file.parent().ok_or(anyhow!("Could not get parent"))?;
            let canon_parent = canonicalize(parent)?;
            Command::new(format!("./{}", name.display()))
                .current_dir(canon_parent)
                .spawn()?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let script_dir = PathBuf::from("test-dirs/1");
    run_scripts(&script_dir)?;
    Ok(())
}

/*

[x] Looks for all files in a directory recursively

[x] Runs the files with `./FILE_NAME` if they are
executable

[x] script files are run from within their own
directory

https://doc.rust-lang.org/std/fs/fn.canonicalize.html

https://doc.rust-lang.org/std/process/struct.Command.html#method.current_dir

https://docs.rs/permissions/latest/permissions/index.html


*/
