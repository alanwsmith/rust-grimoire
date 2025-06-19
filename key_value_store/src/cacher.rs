use anyhow::Result;
use redb::{Database, TableDefinition};
use std::path::PathBuf;

pub struct Cacher<'a> {
  db: Database,
  table:
    TableDefinition<'a, &'static str, &'static str>,
}

impl Cacher<'_> {
  pub fn new(
    db_path: &PathBuf
  ) -> Result<Cacher<'static>> {
    let db = Database::create(db_path)?;
    let table: TableDefinition<&str, &str> =
      TableDefinition::new("storage_data");
    Ok(Cacher { db, table })
  }

  pub fn get_item(
    &self,
    key: &str,
  ) -> Result<Option<String>> {
    let read_txn = self.db.begin_read()?;
    let table = read_txn.open_table(self.table)?;
    match table.get(key)? {
      Some(ag) => Ok(Some(ag.value().to_string())),
      None => Ok(None),
    }
  }

  pub fn save_item(
    &self,
    key: &str,
    value: &str,
  ) -> Result<()> {
    let write_txn = self.db.begin_write()?;
    {
      let mut table =
        write_txn.open_table(self.table)?;
      table.insert(key, value)?;
    }
    write_txn.commit()?;
    Ok(())
  }
}
