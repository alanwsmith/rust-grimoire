use minijinja::context;
use minijinja::Environment;
use minijinja::Source;
use serde::Serialize;
use std::fs;

#[derive(Debug, Serialize)]
struct Payload {
    data: Option<String>,
}

fn make_env<'a>(templates: Vec<(&'a str, &'a str)>) -> Environment<'a> {
    let mut env = Environment::new();
    let mut source = Source::new();
    for i in templates {
        let file_string = fs::read_to_string(i.1).unwrap();
        source.add_template(i.0, file_string).unwrap();
    }
    env.set_source(source);
    env
}

fn main() {
    let templates: Vec<(&str, &str)> =
        vec![("example", "src/bin/minijinja_structure_template.html")];
    let payload = Payload {
        data: Some("Minijinja".to_string()),
    };
    let env = make_env(templates);
    let output = render_template(payload, env);
    dbg!(output);
}

fn render_template(payload: Payload, env: Environment) -> String {
    let tmpl = env.get_template("example").unwrap();
    tmpl.render(context!(payload => &payload))
        .unwrap()
        .to_string()
}
