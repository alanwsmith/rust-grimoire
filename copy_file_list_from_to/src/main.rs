use anyhow::Result;
use std::fs;
use std::path::PathBuf;

#[allow(unused)]
pub fn copy_file_list_from_to(
    files: &Vec<PathBuf>,
    from: &PathBuf,
    to: &PathBuf,
    overwrite: bool,
) -> Result<()> {
    for file in files.iter() {
        let mut do_copy = true;
        let out_path = to.join(file);
        if out_path.exists() && !overwrite {
            do_copy = false;
        }
        if do_copy {
            let in_path = from.join(file);
            let out_parent = out_path.parent().unwrap();
            fs::create_dir_all(&out_parent)?;
            let data = std::fs::read(in_path)?;
            std::fs::write(out_path, &data)?;
        }
    }
    Ok(())
}

// [] Add test files to this
//
// [x] Takes a list of relative file paths and
// copies them from the source directory to
// the destination directory
//
// [x] The copy is done by reading and writing
// the file instead of fs::copy. That's done
// because fs::copy triggers a watchexec/notify
// event on macs. I haven't found a way to filter
// those out so they don't look like content
// updates.

fn main() -> Result<()> {
    println!("Run test for example");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::{Result, anyhow};
    use itertools::Itertools;
    use std::path::PathBuf;
    use walkdir::WalkDir;

    #[test]
    fn run_example() -> Result<()> {
        let from = PathBuf::from("test-dirs/1/input");
        let include_files = Some(vec!["txt"]);
        let to = PathBuf::from("test-dirs/1/output");
        let files = get_files_in_tree(&from, include_files, None)?;
        copy_file_list_from_to(&files, &from, &to, false)?;
        Ok(())
    }

    fn get_files_in_tree(
        dir: &PathBuf,
        with: Option<Vec<&str>>,
        without: Option<Vec<&str>>,
    ) -> Result<Vec<PathBuf>> {
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
            .filter_map(|path| match path.strip_prefix(dir) {
                Ok(p) => Some(p.to_path_buf()),
                Err(_) => None,
            })
            .filter(|pb| {
                pb.components()
                    .find(|part| part.as_os_str().to_string_lossy().starts_with("."))
                    .is_none()
            })
            .filter(|pb| {
                if let Some(with) = &with {
                    if let Some(ext) = pb.extension() {
                        return with
                            .iter()
                            .map(|e| e.to_lowercase())
                            .contains(&ext.to_str().unwrap().to_lowercase());
                    }
                }
                true
            })
            .filter(|pb| {
                if let Some(without) = &without {
                    if let Some(ext) = pb.extension() {
                        return !without
                            .iter()
                            .map(|e| e.to_lowercase())
                            .contains(&ext.to_str().unwrap().to_lowercase());
                    }
                }
                true
            })
            .sorted()
            .collect();
        Ok(files)
    }
}
