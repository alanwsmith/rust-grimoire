use walkdir::WalkDir;

pub fn sort_with_walkdir() {
    for entry in WalkDir::new("src")
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
    {
        dbg!(entry.path().display());
    }
}

#[cfg(test)]

mod test {
    use super::*;
    #[test]
    pub fn test_dir_sorting() {
        sort_with_walkdir();
        assert_eq!(1, 1);
    }
}
