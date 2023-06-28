fn main() {
    let alfa = "this is the thing with the dot.";

    let bravo = alfa.chars().last();

    dbg!(bravo.unwrap());

    // the original doesn't get changed:

    dbg!(alfa);
}
