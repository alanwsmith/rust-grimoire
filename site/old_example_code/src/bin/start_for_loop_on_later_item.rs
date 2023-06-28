fn main() {
    let source = vec!["alfa", "bravo", "charlie", "delta"];
    source.iter().skip(2).for_each(|x| println!("{}", x));
}
