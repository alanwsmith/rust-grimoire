use anyhow::{Result, anyhow};
use itertools::Itertools;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn get_files_in_tree(
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

// [x] Returns a list of files from a directory
// and its sub-directories
//
// [x] The initial directory is removed so the
// paths are relative to the starting location
//
// [x] Files do not have to have extensions
//
// [x] Hidden files are skipped
//
// [x] Hidden directories are skipped
//
// [x] Returns an error if the directory does
// not exist
//
// [x] Option second argument will only include files
// with the listed extensions
//
// [x] Option third argument sill only include files
// that do not have one of the listed extensions
//
// [x] File extensions are case insensitive

fn main() -> Result<()> {
    // let dir = PathBuf::from("test-dirs/1");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn files_with_extensions() {
        let dir = PathBuf::from("test-data/1");
        let left = get_files_in_tree(&dir, None, None).unwrap();
        let right = vec![PathBuf::from("alfa.txt")];
        assert_eq!(left, right);
    }

    #[test]
    fn files_without_extensions() {
        let dir = PathBuf::from("test-data/2");
        let left = get_files_in_tree(&dir, None, None).unwrap();
        let right = vec![PathBuf::from("no-extension")];
        assert_eq!(left, right);
    }

    #[test]
    fn hidden_files_removed() {
        let dir = PathBuf::from("test-data/3");
        let left = get_files_in_tree(&dir, None, None).unwrap();
        let right = vec![PathBuf::from("bravo.txt")];
        assert_eq!(left, right);
    }

    #[test]
    fn sub_dir_files() {
        let dir = PathBuf::from("test-data/4");
        let left = get_files_in_tree(&dir, None, None).unwrap();
        let right = vec![
            PathBuf::from("delta.txt"),
            PathBuf::from("sub/sub-sub/delta-sub-sub.txt"),
        ];
        assert_eq!(left, right);
    }

    #[test]
    fn hidden_dirs_removed() {
        let dir = PathBuf::from("test-data/5");
        let left = get_files_in_tree(&dir, None, None).unwrap();
        let right = vec![PathBuf::from("echo.txt")];
        assert_eq!(left, right);
    }

    #[test]
    fn include_files() {
        let dir = PathBuf::from("test-data/6");
        let with = Some(vec!["html"]);
        let left = get_files_in_tree(&dir, with, None).unwrap();
        let right = vec![
            PathBuf::from("bravo.html"),
            PathBuf::from("sub-dir/sub-sub-dir/charlie.html"),
        ];
        assert_eq!(left, right);
    }

    #[test]
    fn include_files_case_insensitive() {
        let dir = PathBuf::from("test-data/6");
        let with = Some(vec!["HTML"]);
        let left = get_files_in_tree(&dir, with, None).unwrap();
        let right = vec![
            PathBuf::from("bravo.html"),
            PathBuf::from("sub-dir/sub-sub-dir/charlie.html"),
        ];
        assert_eq!(left, right);
    }

    #[test]
    fn exclude_files() {
        let dir = PathBuf::from("test-data/6");
        let without = Some(vec!["html"]);
        let left = get_files_in_tree(&dir, None, without).unwrap();
        let right = vec![
            PathBuf::from("alfa.txt"),
            PathBuf::from("sub-dir/sub-sub-dir/delta.txt"),
        ];
        assert_eq!(left, right);
    }

    #[test]
    fn exclude_files_case_insensitive() {
        let dir = PathBuf::from("test-data/6");
        let without = Some(vec!["HTML"]);
        let left = get_files_in_tree(&dir, None, without).unwrap();
        let right = vec![
            PathBuf::from("alfa.txt"),
            PathBuf::from("sub-dir/sub-sub-dir/delta.txt"),
        ];
        assert_eq!(left, right);
    }

    #[test]
    fn error_on_missing_dir() {
        let dir = PathBuf::from("test-data/invalid-dir");
        let left = get_files_in_tree(&dir, None, None);
        assert!(left.is_err());
    }
}
