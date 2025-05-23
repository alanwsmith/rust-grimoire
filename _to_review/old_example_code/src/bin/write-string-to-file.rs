use std::fs;

fn main() {
    let text = String::from("bravo");
    match write_output(text) {
        Ok(()) => println!("Wrote the file"),
        Err(e) => println!("Error: {}", e),
    }
}

fn write_output(text: String) -> std::io::Result<()> {
    fs::write("output.txt", text)?;
    Ok(())
}

// use std::fs::File;
// use std::io::prelude::*;

// fn main() {
//     let text = String::from("the quick brown fox");
//     match output(text) {
//         Ok(()) => println!("Ok"),
//         Err(e) => println!("Error: {}", e),
//     };
// }

// fn output(text: String) -> std::io::Result<()> {
//     let file_path = "output.txt";
//     let mut file = File::create("outputs.txt")?;
//     file.write(text.as_bytes())?;
//     Ok(())
// }
