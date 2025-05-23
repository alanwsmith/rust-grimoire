fn main() {
    let parts = "before 123 after".split("123");

    // for part in parts {
    //     println!("{}", part)
    // }

    // let collection = parts.collect::<Vec<&str>>();
    // dbg!(collection);

    let collection: Vec<&str> = parts.collect();
    dbg!(collection);

    //
    // let vec = split.collect::<Vec<&str>>();

    // let parts = "some string 123 ffd".split("123");
    // for s in parts {
    //     println!("{}", s)
    // }
    // let vec = &parts.collect::<Vec<&str>>();

    // for part in &parts {
    //     println!("{}", part)
    // }
    // let collection = parts.collect::<Vec<&str>>();
    // dbg!(collection);

    // let collection: Vec<&str> = parts.collect();
    // dbg!(collection);
}
