use keyring::Entry;

// Via:
// https://docs.rs/keyring/latest/keyring/

fn main() {
    let entry =
        Entry::new("test--example--key", "alan")
            .unwrap()
            .get_password()
            .unwrap();

    println!("The password is: {}", entry);
}
