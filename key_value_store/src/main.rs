use anyhow::Result;
use grimoire_example::cacher::Cacher;
use std::path::PathBuf;

fn main() -> Result<()> {
  let db_path = PathBuf::from("storage.redb");
  let cacher = Cacher::new(&db_path)?;
  cacher
    .save_item("alfa", "this is the value of alfa")?;
  let found = cacher.get_item("alfa")?;
  match found {
    Some(result) => {
      println!("Found: {}", result);
    }
    None => {
      println!("Did not find anything");
    }
  }
  Ok(())
}
