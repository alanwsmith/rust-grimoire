use rusqlite::{Connection, Result};

fn table_exists(conn: Connection, table_name: &str) -> Result<bool> {
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master 
        WHERE type='table' AND name=?1",
    )?;
    let rows = stmt.query_map([table_name], |_| Ok(()))?;
    if rows.count() == 1 {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[cfg(test)]
mod text {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn solo_table_does_not_exist() {
        let conn = Connection::open_in_memory().unwrap();
        let expected = false;
        let result = table_exists(conn, "alfa").unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    fn solo_table_exists() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("CREATE TABLE bravo (placeholder TEXT)", ())
            .unwrap();
        let expected = true;
        let result = table_exists(conn, "bravo").unwrap();
        assert_eq!(expected, result);
    }
}
