use imagesize::size;

fn main() {
    let img = size("image_test/tmp.png").unwrap();
    dbg!(img.width);
    dbg!(img.height);
}
