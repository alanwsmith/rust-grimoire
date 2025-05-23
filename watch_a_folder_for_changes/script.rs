#!/usr/bin/env cargo -Zscript

---
[dependencies]
anyhow = "1.0.98"
notify = "8.0.0"
notify-debouncer-full = "0.5.0"
rusqlite = "0.35.0"
sha2 = "0.10.9"

[grimoire]
created = "2025-05-21T17:54:09-04:00"
updated = "2025-05-21T17:54:09-04:00"
id = "2x/v4/9l/ge"
status = "scratch"
---


#![allow(unused)]
use anyhow::Result;
use notify_debouncer_full::*;
use std::time::Duration;
use notify::RecursiveMode;
use notify::Event;
use std::path::PathBuf;
use std::sync::mpsc;
use rusqlite::Connection;

fn main() -> Result<()> {
    watch_folder(&PathBuf::from("../"))?;
    Ok(())
}

fn watch_folder(path: &PathBuf) -> Result<()> {
    println!("Starting watcher");
    let conn = Connection::open_in_memory()?;
    init_db(&conn)?;

        // let create_table_sql = "CREATE TABLE IF NOT EXISTS
        //     files (
        //         path TEXT PRIMARY KEY,
        //         hash TEXT NOT NULL 
        //     )";
        // conn.execute(create_table_sql, ())?;

    let (tx, rx) = mpsc::channel::<Result<Vec<DebouncedEvent>, Vec<notify::Error>>>();
    let mut debouncer = new_debouncer(
        Duration::from_millis(100),
        None,
        tx
    )?;
    debouncer.watch(path, RecursiveMode::Recursive)?;
    for res in rx {
        match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    Ok(())
}

fn init_db(conn: &Connection)  -> Result<()> {
    Ok(())
}


// async fn run_server() -> Result<()> {
//     let ds = DirServer::new()?;
//     ds.load_files()?;
//     let livereload = LiveReloadLayer::new();
//     let reloader = livereload.reloader();
//     let service = ServeDir::new(&ds.dir)
//         .append_index_html_on_directories(true)
//         .not_found_service(get(|| missing_page()));
//     let app = Router::new().fallback_service(service).layer(livereload);
//     let mut debouncer = new_debouncer(
//         Duration::from_millis(100),
//         None,
//         move |result: DebounceEventResult| {
//             if let Some(path) = ds.process_event(result) {
//                 println!("Reload via: {}", path.display());
//                 reloader.reload();
//             }
//         },
//     )?;
//     debouncer.watch(".", RecursiveMode::Recursive)?;
//     if let Some(port) = free_local_port_in_range(5444..=6000) {
//         println!("Starting folder server on port {}", port);
//         let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
//             .await
//             .unwrap();
//         // TODO: handle error here if the server can't start
//         axum::serve(listener, app).await.unwrap();
//     } else {
//         println!(
//             "Could not find an avialable port between {} and {}",
//             minPort, maxPort
//         );
//     }
//     Ok(())
// }
