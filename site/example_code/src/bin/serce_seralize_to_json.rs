use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Widget {
    alfa: String,
    bravo: Vec<String>,
    charlie: Thing,
    delta: Thing 
}

#[derive(Serialize, Deserialize)]
enum Thing {
   Arch,
   Book { title: String },
}

fn main() {
    let json = make_json();
    match json {
        Ok(string) => { println!("{}", string); }
        Err(e) => { dbg!(e); }
    }
}

fn make_json() -> Result<String> {
    let w = Widget {
        alfa: "Hello".to_string(),
        bravo: vec!["world".to_string(), "folks".to_string()],
        charlie: Thing::Arch,
        delta: Thing::Book { title: "Dune".to_string() },
    };

    serde_json::to_string_pretty(&w)
}

