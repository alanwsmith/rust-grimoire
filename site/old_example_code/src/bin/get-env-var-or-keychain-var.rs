use keyring::error::Error;
use keyring::Entry;
use std::env;

// This function takes the name of an env var and
// a credentail key and username as arguments.
// It returns the env var if it finds it. Otherwise
// it tries to get the credentail from the system
// storage. If it finds it, it returns it from there,
// if not it returns an error.
//
// The result in both cases is a Result that will
// have either Ok with the returned value or
// an NoEntry Err from keyring

fn get_var(
    envkey: &str, credkey: &str, creduser: &str,
) -> Result<String, Error> {
    if let Ok(value) = env::var(envkey) {
        Ok(value)
    }
    else {
        if let Ok(entry) =
            Entry::new(credkey, creduser)
        {
            if let Ok(value) = entry.get_password()
            {
                Ok(value)
            }
            else {
                Err(Error::NoEntry)
            }
        }
        else {
            Err(Error::NoEntry)
        }
    }
}

fn main() {
    let value = get_var(
        "TESTENV_NAME2",
        "test--example--key",
        "alan",
    );
    match value {
        Ok(v) => println!("Got {v}"),
        Err(e) => println!("Error: {e}"),
    }
}

// TODO: Figure out how to add Ambiguous
// error here too:
// https://docs.rs/keyring/latest/keyring/error/enum.Error.html#variant.Ambiguous
// sounds like it's rarely a problem, but is something
// that should be addressed. Or, maybe just passing no
// entry is fine?
