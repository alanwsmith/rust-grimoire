use anyhow::Error;
use glob::glob;
use std::fs;
use std::path::PathBuf;

fn main() {
    let input_root = "test_input_dir";
    let input_extension = "txt";
    let output_root = "test_output_dir";
    let output_extension = "neo";
    let file_list =
        prep_structure(input_root, input_extension, output_root, output_extension).unwrap();
    dbg!(file_list);
}

fn prep_structure(
    in_root: &str,
    in_ext: &str,
    out_root: &str,
    out_ext: &str,
) -> Result<Vec<(PathBuf, PathBuf)>, Error> {
    let mut response: Vec<(PathBuf, PathBuf)> = vec![];
    let mut glob_expression = String::from(in_root);
    glob_expression.push_str("/**/*.");
    glob_expression.push_str(in_ext);
    for entry in glob(glob_expression.as_str())? {
        let in_path = entry?;
        let mut out_path = PathBuf::from(out_root);
        out_path.push(in_path.strip_prefix(in_root).unwrap());
        out_path.set_extension(out_ext);
        let check_dir = &out_path.parent().unwrap();
        if !check_dir.try_exists().unwrap() {
            fs::create_dir_all(check_dir).unwrap();
        }
        response.push((in_path, out_path));
    }
    Ok(response)
}
