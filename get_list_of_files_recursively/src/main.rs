use anyhow::Result;
use itertools::Itertools;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() -> Result<()> {
    // let dir = PathBuf::from("test-dirs/1");
    Ok(())
}

// - Returns a list of files from a directory
// and its sub-directories
//
// - The initial directory is removed so the
// paths are relative to the starting location
//
// - Files do not have to have extensions
//
//

pub fn get_file_list(dir: &PathBuf) -> Result<Vec<PathBuf>> {
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
        .filter(|pb| !pb.display().to_string().starts_with("."))
        .sorted()
        .collect();
    Ok(files)
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn files_with_extensions() {
        let dir = PathBuf::from("test-data/1");
        let left = get_file_list(&dir).unwrap();
        let right = vec![PathBuf::from("alfa.txt")];
        assert_eq!(left, right);
    }

    #[test]
    fn files_without_extensions() {
        let dir = PathBuf::from("test-data/2");
        let left = get_file_list(&dir).unwrap();
        let right = vec![PathBuf::from("no-extension")];
        assert_eq!(left, right);
    }

    #[test]
    fn hidden_files_removed() {
        let dir = PathBuf::from("test-data/3");
        let left = get_file_list(&dir).unwrap();
        let right = vec![PathBuf::from("bravo.txt")];
        assert_eq!(left, right);
    }

    #[test]
    fn sub_dir_files() {
        let dir = PathBuf::from("test-data/4");
        let left = get_file_list(&dir).unwrap();
        let right = vec![
            PathBuf::from("delta.txt"),
            PathBuf::from("sub/sub-sub/delta-sub-sub.txt"),
        ];
        assert_eq!(left, right);
    }
}
