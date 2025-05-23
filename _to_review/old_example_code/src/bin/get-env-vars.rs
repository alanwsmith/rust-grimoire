use std::env;

// You'll need to set the env var for this to
// work with something like:
// export TESTING_VAR="alfa"

// TODO: Look at: envvar

fn main() {
    println!("running");
    for (key, value) in env::vars() {
        if key == "TESTING_VAR" {
            println!("{}", value);
        }
    }
}
