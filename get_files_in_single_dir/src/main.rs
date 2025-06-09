use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub fn get_files_in_dir(
    dir: &PathBuf,
    with: Option<Vec<&str>>,
    without: Option<Vec<&str>>,
) -> Result<Vec<PathBuf>> {
    Ok(fs::read_dir(dir)?
        .into_iter()
        .filter_map(|path| path.ok())
        .map(|path| path.path().to_path_buf())
        .filter(|path| path.is_file())
        .filter_map(|path| match path.strip_prefix(dir) {
            Ok(p) => Some(p.to_path_buf()),
            Err(_) => None,
        })
        .filter(|path| !path.display().to_string().starts_with("."))
        .filter(|path| {
            if let Some(with) = &with {
                if let Some(ext) = path.extension() {
                    with.contains(&ext.to_str().unwrap())
                } else {
                    false
                }
            } else {
                true
            }
        })
        .filter(|path| {
            if let Some(without) = &without {
                if let Some(ext) = path.extension() {
                    !without.contains(&ext.to_str().unwrap())
                } else {
                    false
                }
            } else {
                true
            }
        })
        .collect())
}

// [x] Read files in a single directory without
// descending into sub directories
//
// [x] Does not return directories inside the
// search dir
//
// [x] Strips initial directory so paths are
// relative
//
// [x] Skips hidden files
//
// [x] Takes Option with vec for file extensions
// to include.
//
// [x] Takes Option with vec for file extensions
// to exclude.

fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn one_file_test() {
        let dir = PathBuf::from("test-dirs/1");
        let left = get_files_in_dir(&dir, None, None).unwrap();
        let right = vec![PathBuf::from("alfa.txt")];
        assert_eq!(left, right);
    }

    #[test]
    fn do_no_decend_into_sub_dirs() {
        let dir = PathBuf::from("test-dirs/2");
        let left = get_files_in_dir(&dir, None, None).unwrap();
        let right = vec![PathBuf::from("bravo.txt")];
        assert_eq!(left, right);
    }

    #[test]
    fn skip_hidden_files() {
        let dir = PathBuf::from("test-dirs/3");
        let left = get_files_in_dir(&dir, None, None).unwrap();
        let right = vec![PathBuf::from("charlie.txt")];
        assert_eq!(left, right);
    }

    #[test]
    fn only_include_files() {
        let dir = PathBuf::from("test-dirs/4");
        let include_files = Some(vec!["html"]);
        let left = get_files_in_dir(&dir, include_files, None).unwrap();
        let right = vec![PathBuf::from("bravo.html")];
        assert_eq!(left, right);
    }

    #[test]
    fn exclude_files() {
        let dir = PathBuf::from("test-dirs/5");
        let exclude_files = Some(vec!["html"]);
        let left = get_files_in_dir(&dir, None, exclude_files).unwrap();
        let right = vec![PathBuf::from("charlie.txt")];
        assert_eq!(left, right);
    }
}
