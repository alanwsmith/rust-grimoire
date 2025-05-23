use std::io;


fn main() {

    println!("Type something and hit enter:");

    let mut input_value = String::new();

    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read line");

    println!("You typed: {input_value}");


}
