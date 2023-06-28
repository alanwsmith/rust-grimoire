// use image::io::Reader;

// fn main() {
//     let img = Reader::open("/Users/alan/workshop/__sources/tmp/pixel_frame.png")
//         .unwrap()
//         .decode()
//         .unwrap();
//     let width = img.width();
//     let height = img.height();
//     dbg!(width);
//     dbg!(height);
// }

use std::process::Command;

fn main() {
    let path = "/Users/alan/workshop/__sources/tmp/pixel_frame.png";
    let height = image_height(path);
    dbg!(height);
}

fn image_height(img: &str) -> u32 {
    let args = ["-format", "%h", img];
    let response = Command::new("identify").args(args).output().unwrap();
    let height_string = String::from_utf8(response.stdout).unwrap();
    height_string.parse::<u32>().unwrap()
}
