use rusqlite::Connection;
use rusqlite::Result;

// fn main() -> Result<()> {
//     let conn = Connection::open("./_sqlite_db_test.db3")?;
//     let create_table = "
//         CREATE TABLE storage (name TEXT, color TEXT)";
//     conn.execute(create_table, ())?;
//     Ok(())
// }

// fn main() -> Result<()> {
//     let conn = Connection::open("./_sqlite_db_test.db3")?;
//     let insert_data = "
//         INSERT INTO storage (name, color) VALUES (?1, ?2)";
//     conn.execute(insert_data, ("alfa", "red"))?;
//     Ok(())
// }

fn main() -> Result<()> {
    let conn = Connection::open("./_sqlite_db_test.db3")?;
    let mut get_data = conn.prepare("SELECT name, color FROM storage WHERE name = ?1")?;
    let response = get_data.query_map(["alfa"], |row| {
        Ok((row.get::<usize, String>(0), row.get::<usize, String>(1)))
    })?;
    for item in response {
        println!("{}", item.unwrap().0.unwrap());
    }
    Ok(())
}
