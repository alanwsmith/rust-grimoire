#![allow(unused)]
use anyhow::Result;
use minijinja::syntax::SyntaxConfig;
use minijinja::{Environment, Value, context, path_loader};
use std::path::PathBuf;

// This is designed to create a log of errors
// and output to a file location even if the
// template has an error (in which case it
// outputs the error message)

pub struct Renderer<'a> {
    pub env: Environment<'a>,
    pub log: Vec<RendererStatus>,
}

impl Renderer<'_> {
    pub fn new() -> Renderer<'static> {
        let mut renderer = Renderer {
            env: Environment::new(),
            log: vec![],
        };
        renderer.env.set_syntax(
            SyntaxConfig::builder()
                .block_delimiters("[!", "!]")
                .variable_delimiters("[@", "@]")
                .comment_delimiters("[#", "#]")
                .build()
                .unwrap(),
        );
        renderer
    }

    pub fn add_template(&mut self, name: &str, content: &str) {
        match self
            .env
            .add_template_owned(name.to_string(), content.to_string())
        {
            Ok(_) => self.log.push(RendererStatus::TemplateLoadSuccess {
                path: None,
                name: name.to_string(),
            }),
            Err(e) => {
                self.env
                    .add_template_owned(name.to_string(), Renderer::error_template(&e.to_string()))
                    .unwrap();
                self.log.push(RendererStatus::TemplateLoadError {
                    path: None,
                    name: name.to_string(),
                    error_text: e.to_string(),
                })
            }
        }
    }

    pub fn add_template_dir(&mut self, dir: &PathBuf) {
        if dir.is_dir() {
            self.env.set_loader(path_loader(dir));
            self.log.push(RendererStatus::DirectoryLoadSuccess {
                path: dir.to_path_buf(),
            });
        } else {
            self.log.push(RendererStatus::DirectoryLoadError {
                path: dir.to_path_buf(),
                error_text: format!(
                    "Tried to load tempaltes from missing directory: {}",
                    dir.display()
                ),
            });
        }
    }

    fn error_template(error_text: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<style>
body {{ background-color: black; color: #aaa; }}
</style>
</head>
<body>{}</body>
"#,
            error_text
        )
    }

    pub fn errors(&self) -> Vec<&RendererStatus> {
        self.log
            .iter()
            .filter(|item| match item {
                RendererStatus::TemplateLoadError { .. } => true,
                RendererStatus::DirectoryLoadError { .. } => true,
                _ => false,
            })
            .collect()
    }
}

pub enum RendererStatus {
    TemplateLoadSuccess {
        path: Option<PathBuf>,
        name: String,
    },
    TemplateLoadError {
        path: Option<PathBuf>,
        name: String,
        error_text: String,
    },
    DirectoryLoadSuccess {
        path: PathBuf,
    },
    DirectoryLoadError {
        path: PathBuf,
        error_text: String,
    },
}

fn main() -> Result<()> {
    println!("Check tests for output");
    Ok(())
}

// pub fn add_mj_template(env: &mut Environment, name: &str, content: &str) -> Result<()> {
//     env.add_template_owned(name.to_string(), content.to_string())
//         .unwrap();
//     Ok(())
// }

// pub fn output_content_or_error(env: &Environment, template_name: &str, data: &Value) -> Result<()> {
//     let template = env.get_template(template_name).unwrap();
//     let output = template.render(context!(data)).unwrap();
//     println!("{}", output);
//     Ok(())
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_template_without_error() {
        let mut renderer = Renderer::new();
        renderer.add_template("example", "asdf");
        assert!(renderer.errors().len() == 0);
    }

    #[test]
    fn load_template_with_error() {
        let mut renderer = Renderer::new();
        renderer.add_template("example", "[@ asdf");
        assert!(renderer.errors().len() == 1);
    }

    #[test]
    fn load_directory_that_exists() {
        let mut renderer = Renderer::new();
        let template_dir = PathBuf::from("test-dir/1/valid-templates");
        renderer.add_template_dir(&template_dir);
        assert!(renderer.errors().len() == 0);
    }

    #[test]
    fn load_directory_that_does_not_exist() {
        let mut renderer = Renderer::new();
        let template_dir = PathBuf::from("invalid_directory");
        renderer.add_template_dir(&template_dir);
        assert!(renderer.errors().len() == 1);
    }

    // fn get_env() -> Environment<'static> {
    //     let env = Environment::new();
    //     env
    // }
    // add_mj_template(env, name, content);
    // let template_name = "example";
    // let data = Value::from_serialize(vec!["alfa", "bravo"]);
    // let _ = output_content_or_error(&env, template_name, &data);
}
