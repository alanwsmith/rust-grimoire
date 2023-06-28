use dirs;

fn main() {
    let mut dir_path = dirs::home_dir().unwrap();
    dir_path.push("workshop");
    dbg!(dir_path);
}
