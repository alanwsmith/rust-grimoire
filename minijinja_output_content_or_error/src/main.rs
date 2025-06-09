use anyhow::Result;
use minijinja::syntax::SyntaxConfig;
use minijinja::{Environment, Value, context, path_loader};
use std::fs;
use std::path::PathBuf;

// This is designed to create a log of errors
// and output to a file location even if the
// template has an error (in which case it
// outputs the error message)
//
// [] Always return a string for rendering
// with various fallbacks if things go
// wrong at any stage

pub struct Renderer<'a> {
    pub env: Environment<'a>,
    pub log: Vec<RendererStatus>,
    error_template: Option<String>,
}

impl Renderer<'_> {
    pub fn new() -> Renderer<'static> {
        let mut renderer = Renderer {
            env: Environment::new(),
            log: vec![],
            error_template: None,
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
            Ok(_) => self.log.push(RendererStatus::AddTemplateSuccess {
                path: None,
                name: name.to_string(),
            }),
            Err(e) => {
                self.env
                    .add_template_owned(name.to_string(), self.error_template(&e.to_string()))
                    .unwrap();
                self.log.push(RendererStatus::AddTemplateError {
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
            self.log.push(RendererStatus::AddTemplateDirSuccess {
                path: dir.to_path_buf(),
            });
        } else {
            self.log.push(RendererStatus::AddTemplateDirError {
                path: dir.to_path_buf(),
                error_text: format!(
                    "Tried to load tempaltes from missing directory: {}",
                    dir.display()
                ),
            });
        }
    }

    pub fn add_template_from_path(&mut self, name: &str, path: &PathBuf) {
        match fs::read_to_string(path) {
            Ok(content) => {
                self.add_template(name, &content);
            }
            Err(e) => {
                self.log.push(RendererStatus::AddTemplateFileError {
                    name: name.to_string(),
                    path: path.to_path_buf(),
                    error_text: format!("Error: {} - on file: {}", e.to_string(), path.display()),
                });
            }
        }
    }

    fn error_template(&self, error_text: &str) -> String {
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
                RendererStatus::AddTemplateError { .. } => true,
                RendererStatus::AddTemplateDirError { .. } => true,
                RendererStatus::AddTemplateFileError { .. } => true,
                RendererStatus::GetTemplateError { .. } => true,
                RendererStatus::RenderContentError { .. } => true,
                _ => false,
            })
            .collect()
    }

    pub fn render_content(&mut self, template: &str, context: &Value) -> String {
        match self.env.get_template(template) {
            Ok(tmpl) => match tmpl.render(context) {
                Ok(output) => {
                    self.log.push(RendererStatus::RenderContentSuccess {
                        template: template.to_string(),
                    });
                    output
                }
                Err(e) => {
                    self.log.push(RendererStatus::RenderContentError {
                        error_text: e.to_string(),
                        template: template.to_string(),
                    });
                    self.error_template(&e.to_string())
                }
            },
            Err(e) => {
                self.log.push(RendererStatus::GetTemplateError {
                    template: template.to_string(),
                    error_text: e.to_string(),
                });
                self.error_template(&e.to_string())
            }
        }
    }

    pub fn set_error_template(&mut self, fmt: &str) {
        self.error_template = Some(fmt.to_string());
    }
}

pub enum RendererStatus {
    AddTemplateError {
        path: Option<PathBuf>,
        name: String,
        error_text: String,
    },
    AddTemplateFileError {
        path: PathBuf,
        name: String,
        error_text: String,
    },
    AddTemplateSuccess {
        path: Option<PathBuf>,
        name: String,
    },
    AddTemplateDirError {
        path: PathBuf,
        error_text: String,
    },
    AddTemplateDirSuccess {
        path: PathBuf,
    },
    GetTemplateError {
        template: String,
        error_text: String,
    },
    RenderContentError {
        template: String,
        error_text: String,
    },
    RenderContentSuccess {
        template: String,
    },
}

fn main() -> Result<()> {
    println!("Check tests for output");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

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

    #[test]
    fn basic_content_generation_just_return_template() {
        let mut renderer = Renderer::new();
        renderer.add_template("test-alfa", "test-alfa-content");
        let context = context!();
        let left = renderer.render_content("test-alfa", &context);
        let right = "test-alfa-content".to_string();
        assert_eq!(left, right);
    }

    #[test]
    fn render_files_because_template_can_not_load() {
        let mut renderer = Renderer::new();
        renderer.set_error_template("{}");
        let context = context!();
        let _left = renderer.render_content("missing-template", &context);
        let _right = "some-error-stuff".to_string();
        assert!(renderer.errors().len() == 1);
        // TODO: Figure out how to test the return easily
        // assert_eq!(left, right);
    }

    #[test]
    fn render_fails_at_output() {
        let mut renderer = Renderer::new();
        let context = context!();
        let template_dir = PathBuf::from("test-dir/3");
        renderer.add_template_dir(&template_dir);
        let _left = renderer.render_content("nested-data.neoj", &context);
        let _right = "some-error-stuff".to_string();
        assert!(renderer.errors().len() == 1);
        // TODO: Figure out how to test the return easily
        // assert_eq!(left, right);
    }

    #[test]
    fn add_template_from_file_success() {
        let mut renderer = Renderer::new();
        let path = PathBuf::from("test-dir/4/file-template.neoj");
        renderer.add_template_from_path("example-from-path", &path);
        assert!(renderer.errors().len() == 0);
    }

    #[test]
    fn add_template_from_file_error() {
        let mut renderer = Renderer::new();
        let path = PathBuf::from("no-file.neoj");
        renderer.add_template_from_path("example-from-path", &path);
        assert!(renderer.errors().len() == 1);
    }

    // TODO: Build context with error output if
    // something freaks out

    /////

    #[test]
    fn todo_load_invalid_template_in_dir() {
        let mut renderer = Renderer::new();
        let template_dir = PathBuf::from("test-dir/2/invalid-templates");
        renderer.add_template_dir(&template_dir);
        // assert!(renderer.errors().len() == 1);
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
