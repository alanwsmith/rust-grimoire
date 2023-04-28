fn main() {
    let x = 6400;
    (0..32).for_each(|n| println!("{}", (x >> n) & 1));
}
