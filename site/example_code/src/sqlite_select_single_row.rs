use rusqlite::{Connection, Result};

// https://docs.rs/rusqlite/0.29.0/rusqlite/struct.Connection.html#method.query_row

fn get_single_row(conn: &Connection, key: &str) -> Result<(String, String)> {
    conn.query_row("SELECT key, value FROM silo WHERE key = ?1", [key], |row| {
        Ok((
            row.get_unwrap::<usize, String>(0).to_string(),
            row.get_unwrap::<usize, String>(1).to_string(),
        ))
    })
}

#[cfg(test)]
mod text {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn solo_table_exists() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute("CREATE TABLE silo (key TEXT, value TEXT)", ())
            .unwrap();
        conn.execute("INSERT INTO silo (key, value) VALUES (?1, ?2)", ["class", "tango"])
            .unwrap();
        let expected = ("class".to_string(), "tango".to_string());
        let result = get_single_row(&conn, "class").unwrap();
        assert_eq!(expected, result);
    }

    //
}
