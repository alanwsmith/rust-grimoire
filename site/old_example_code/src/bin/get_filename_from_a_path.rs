use std::path::PathBuf;

let path = PathBuf::from("/Users/alan/Desktop/here.txt");
println!("{}", path.file_name().unwrap().to_str().unwrap());
