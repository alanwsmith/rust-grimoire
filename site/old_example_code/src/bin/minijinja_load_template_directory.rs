use minijinja::context;
use minijinja::Environment;
use minijinja::Source;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Payload {
    data: Option<String>,
}

fn create_env(path: &str) -> Environment<'static> {
    let mut env = Environment::new();
    env.set_source(Source::from_path(path));
    // NOTE: `templates`` shows up as an empty
    // vec in debug here
    // dbg!(&env);
    env
}

fn main() {
    let env = create_env("./minijinja_templates");
    let payload = Payload {
        data: Some("the quick brown fox".to_string()),
    };
    let output = render_template(payload, env, "some_template.html");
    dbg!(output);
}

fn render_template(payload: Payload, env: Environment, template: &str) -> String {
    let tmpl = env.get_template(template).unwrap();
    tmpl.render(context!(payload => &payload))
        .unwrap()
        .to_string()
}
