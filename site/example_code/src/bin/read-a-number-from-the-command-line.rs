use std::io;

fn main() {
    println!("Type a number and hit enter:");

    loop {
        let mut input_value = String::new();

        io::stdin()
            .read_line(&mut input_value)
            .expect("Failed to read line");

        let the_number: u32 = match input_value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number. Try again: ");
                continue;
            }
        };

        println!("You typed: {the_number}");
        break;
    }
}
