#!/usr/bin/env cargo -Zscript

---
[dependencies]
anyhow = "1.0.98"
itertools = "0.14.0"
notify = "8.0.0"
notify-debouncer-full = "0.5.0"
rusqlite = "0.35.0"
sha2 = "0.10.9"
tokio = {version = "1.45.0", features = ["rt-multi-thread", "macros", "sync"] }

[grimoire]
created = "2025-05-21T17:54:09-04:00"
updated = "2025-05-21T17:54:09-04:00"
id = "2x/v4/9l/ge"
status = "scratch"
---

#![allow(unused)]
use anyhow::Result;
use anyhow::Error;
use notify_debouncer_full::*;
use std::time::Duration;
use notify::RecursiveMode;
use notify::Event;
use std::path::PathBuf;
use rusqlite::Connection;
use tokio::sync::mpsc;
use notify::EventKind;
use itertools::Itertools;
use tokio::runtime::Handle;

struct DirWatcher {
    rx: tokio::sync::mpsc::Receiver<Vec<PathBuf>>
}

impl DirWatcher {
    fn remove_hidden_and_tmp(path: &PathBuf) -> bool {
        let check_string = path.display().to_string();
        if check_string.ends_with("~") {
            false 
        } else if let None = path.components().find(|part| { 
            if let std::path::Component::Normal(x) = part {
                x.to_str().unwrap().starts_with(".")
            } else  {
                false
            }
         }) {
            true
        } else {
            false
        }
    }

    pub fn new(path: &PathBuf) -> Result<DirWatcher> {
        let (tx, rx) = mpsc::channel::<Vec<PathBuf>>(1);
        let send_path = path.clone();
        let tx_bridge = tx.clone();
        tokio::spawn(async move  {
            let rt = Handle::current();
            let (internal_tx, mut internal_rx) = mpsc::channel::<Vec<PathBuf>>(2);
            let internal_tx2 = internal_tx.clone();
             let mut debouncer = new_debouncer(
                 Duration::from_millis(300),
                 None,
                 move |result: DebounceEventResult| {
                    if let Ok(events) = result {
                        let ex: Vec<_> = events.iter().filter_map(|payload|
                            {
                                match &payload.event.kind {
                                    EventKind::Any => {
                                        None
                                    }
                                    EventKind::Access(_) => {
                                        None
                                    }
                                    EventKind::Create(_) => {
                                        Some(&payload.event.paths)
                                    }
                                    EventKind::Modify(change) => {
                                        None
                                        // match change {
                                        //     Name => {
                                        //         Some(&payload.event.paths)
                                        //     }
                                        //     Data => {
                                        //         Some(&payload.event.paths)
                                        //     }
                                        //     _ => None
                                        // }
                                    },
                                    EventKind::Other => {
                                        None
                                    }
                                    EventKind::Remove(_) => {
                                        Some(&payload.event.paths)
                                    }

                                }
                            }
                        )
                            .flatten()
                            .unique()
                            .map(|p| p.to_path_buf())
                            .filter(|p| DirWatcher::remove_hidden_and_tmp(p))
                            .collect();
                        //dbg!(ex);

                        let tx3 = internal_tx.clone();
                        rt.spawn(async move {
                            if let Err(e) = tx3.send(vec![]).await {
                                println!("Error sending event result: {:?}", e);
                            }
                        });

                     //internal_tx2.send(true);
                    }
                     //dbg!(result);
                     //internal_tx2.send(true);
                 }
             ).unwrap();
            debouncer.watch(send_path, RecursiveMode::Recursive).unwrap();
            dbg!("start loop");
            let rt2 = Handle::current();
            while let Some(_) = internal_rx.recv().await {
                dbg!("inside hit");
                //tx_bridge.send(true);
                        let tx4 = tx_bridge.clone();
                        rt2.spawn(async move {
                            if let Err(e) = tx4.send(vec![]).await {
                                println!("Error sending event result: {:?}", e);
                            }
                        });
            }
        });
        let dw = DirWatcher {
            rx,
        };
        Ok(dw)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let dir_to_watch = PathBuf::from("../");
    let mut dw = DirWatcher::new(&dir_to_watch)?;
    while let Some(_) = dw.rx.recv().await {
        dbg!("outside hit");
    }
    Ok(())
}

    // conn: Connection, 
    //debouncer: Debouncer<notify::RecommendedWatcher,notify_debouncer_full::RecommendedCache>,
    //rx: Receiver<Result<bool, Error>>,
    //tx: Sender<Result<bool, Error>>,

        //let (tx, rx) = mpsc::channel::<Result<bool>>();

//        dw.init_db();
//
        //let conn = Connection::open_in_memory()?;

    // fn init_db(&self)  -> Result<()> {
    //     println!("Creating hash database");
    //     let create_table_sql = "CREATE TABLE IF NOT EXISTS
    //         files (
    //             path TEXT PRIMARY KEY,
    //             hash TEXT NOT NULL 
    //         )";
    //     self.conn.execute(create_table_sql, ())?;
    //     Ok(())
    // }

        //debouncer.watch(path, RecursiveMode::Recursive)?;


        // for res in rx {
        //     dbg!("asdf");
        //     // match res {
        //     //     Ok(event) => println!("event: {:?}", event),
        //     //     Err(e) => println!("watch error: {:?}", e),
        //     // }
        // }

        // for res in dw.rx {
        //     dbg!("asdf");
        //     // match res {
        //     //     Ok(event) => println!("event: {:?}", event),
        //     //     Err(e) => println!("watch error: {:?}", e),
        //     // }
        // }

// fn watch_folder(path: &PathBuf) -> Result<()> {
//     println!("Starting watcher");
//     let conn = Connection::open_in_memory()?;
//     init_db(&conn)?;
//     let (tx, rx) = mpsc::channel::<Result<Vec<DebouncedEvent>, Vec<notify::Error>>>();
//     let mut debouncer = new_debouncer(
//         Duration::from_millis(100),
//         None,
//         tx
//     )?;
//     debouncer.watch(path, RecursiveMode::Recursive)?;
//     for res in rx {
//         match res {
//             Ok(event) => println!("event: {:?}", event),
//             Err(e) => println!("watch error: {:?}", e),
//         }
//     }
//     Ok(())
// }



        //let tx2 = tx.clone();
        // let mut debouncer = new_debouncer(
        //     Duration::from_millis(100),
        //     None,
        //     move |result: DebounceEventResult| {
        //         dbg!(result);
        //         tx2.send(Ok(true));
        //         // match result {
        //         //     Ok(events) => events.iter().for_each(|event| println!("{event:?}")),
        //         //     Err(errors) => errors.iter().for_each(|error| println!("{error:?}")),
        //         // }
        //     }
        // )?;
