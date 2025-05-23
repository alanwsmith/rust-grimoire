use minijinja::Environment;

use serde::Serialize;

// https://docs.rs/minijinja/0.3.0/minijinja/syntax/index.html

#[derive(Serialize)]
pub struct Widget {
    alfa: Option<String>,
}

fn main() {
    let w = Widget {
        // alfa: Some(String::from("apple")),
        alfa: None,
    };

    let template = r#"
Test alfa: {% if alfa %}exists{% else %}does not exist{% endif %}
"#;

    let mut env = Environment::new();
    env.add_template("basic", template).unwrap();
    let tmpl = env.get_template("basic").unwrap();
    println!("{}", tmpl.render(w).unwrap());
}
