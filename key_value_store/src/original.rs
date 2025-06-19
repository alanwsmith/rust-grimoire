use anyhow::Result;
use redb::{Database, TableDefinition};
use std::path::PathBuf;

const TABLE: TableDefinition<&str, &str> =
  TableDefinition::new("my_data");

fn main() -> Result<()> {
  let db_file = PathBuf::from("test_db.redb");
  let db = Database::create(&db_file)?;
  save_item(&db, "alfa", "bravo")?;
  let value_from_cache = get_item(&db, "alfa")?;
  dbg!(value_from_cache);
  Ok(())
}

fn save_item(
  db: &Database,
  key: &str,
  value: &str,
) -> Result<()> {
  let write_txn = db.begin_write()?;
  {
    let mut table = write_txn.open_table(TABLE)?;
    table.insert(key, value)?;
  }
  write_txn.commit()?;
  Ok(())
}

fn get_item(
  db: &Database,
  key: &str,
) -> Result<String> {
  let read_txn = db.begin_read()?;
  let table = read_txn.open_table(TABLE)?;
  Ok(table.get(key)?.unwrap().value().to_string())
}
