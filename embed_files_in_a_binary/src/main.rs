use anyhow::Result;
use rust_embed::Embed;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Embed)]
#[folder = "src/alfa_files"]
struct FilePayload;

fn main() -> Result<()> {
    extra_files()?;
    Ok(())
}

fn extra_files() -> Result<()> {
    let output_root = PathBuf::from("test_output");
    for file in FilePayload::iter() {
        let name = file.as_ref();
        let output_path = output_root.join(name);
        if let Some(content) = FilePayload::get(name) {
            if let Some(parent) = output_path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)?
                }
            }
            let body: Vec<u8> = content.data.into();
            let mut output = File::create(output_path)?;
            output.write_all(&body)?;
        }
    }
    Ok(())
}
