use dialoguer::Confirm;

pub fn main() {
    match Confirm::new()
        .with_prompt("Do you want to continue?")
        .interact()
    {
        Ok(status) => {
            if status == true {
                println!("Looks like you want to continue");
            } else {
                println!("nevermind then :(");
            }
        }
        Err(err) => {
            println!("{}", err)
        }
    }
}
