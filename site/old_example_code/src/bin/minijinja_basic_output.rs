use minijinja::Environment;

use serde::Serialize;

// https://docs.rs/minijinja/0.3.0/minijinja/syntax/index.html

#[derive(Serialize)]
pub struct Widget {
    alfa: String,
}

fn main() {
    let w = Widget {
        alfa: String::from("apple"),
    };
    let mut env = Environment::new();
    env.add_template("hello", "Hello {{ alfa }}!")
        .unwrap();
    let tmpl = env.get_template("hello").unwrap();
    println!("{}", tmpl.render(w).unwrap());
}
