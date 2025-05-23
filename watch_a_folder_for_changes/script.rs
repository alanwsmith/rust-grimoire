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
---



use anyhow::Result;
use itertools::Itertools;
use notify::EventKind;
use notify::RecursiveMode;
use notify_debouncer_full::*;
use std::path::PathBuf;
use std::time::Duration;
use tokio::runtime::Handle;
use tokio::sync::mpsc;
use notify::event::ModifyKind;

struct DirWatcher {
    rx: tokio::sync::mpsc::Receiver<Vec<PathBuf>>,
}

impl DirWatcher {
    fn remove_hidden_and_tmp(path: &PathBuf) -> bool {
        let check_string = path.display().to_string();
        if check_string.ends_with("~") {
            false
        } else if let None = path.components().find(|part| {
            if let std::path::Component::Normal(x) = part {
                x.to_str().unwrap().starts_with(".")
            } else {
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
        tokio::spawn(async move {
            let rt = Handle::current();
            let (internal_tx, mut internal_rx) = mpsc::channel::<Vec<PathBuf>>(2);
            let mut debouncer = new_debouncer(
                Duration::from_millis(400),
                None,
                move |result: DebounceEventResult| {
                    if let Ok(events) = result {
                        let paths: Vec<_> = events
                            .iter()
                            .filter_map(|payload| {
                                match &payload.event.kind {
                                    EventKind::Any => None,
                                    EventKind::Access(_) => None,
                                    EventKind::Create(_) => Some(&payload.event.paths),
                                    EventKind::Modify(change) => {
                                        match change {
                                            ModifyKind::Name(_) => {
                                                Some(&payload.event.paths)
                                            }
                                            ModifyKind::Data(_) => {
                                                Some(&payload.event.paths)
                                            }
                                            _ => None
                                        }
                                    }
                                    EventKind::Other => None,
                                    EventKind::Remove(_) => Some(&payload.event.paths),
                                }
                            })
                            .flatten()
                            .unique()
                            .map(|p| p.to_path_buf())
                            .filter(|p| DirWatcher::remove_hidden_and_tmp(p))
                            .collect();
                        if paths.len() > 0 {
                        let tx3 = internal_tx.clone();
                        rt.spawn(async move {
                            if let Err(e) = tx3.send(paths).await {
                                println!("Error sending event result: {:?}", e);
                            }
                        });
                        }

                    }
                },
            )
            .unwrap();
            debouncer
                .watch(send_path, RecursiveMode::Recursive)
                .unwrap();
            let rt2 = Handle::current();
            while let Some(paths) = internal_rx.recv().await {
                let tx4 = tx_bridge.clone();
                rt2.spawn(async move {
                    if let Err(e) = tx4.send(paths).await {
                        println!("Error sending event result: {:?}", e);
                    }
                });
            }
        });
        let dw = DirWatcher { rx };
        Ok(dw)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let dir_to_watch = PathBuf::from("../");
    let mut dw = DirWatcher::new(&dir_to_watch)?;
    while let Some(paths) = dw.rx.recv().await {
        for path in paths {
            println!("Caught change with: {}", path.display());
        }
    }
    Ok(())
}


// -- metadata 
// -- created: 2025-05-21T17:54:09-04:00
// -- updated: 2025-05-21T17:54:09-04:00
// -- id: 2x/v4/9l/ge
// -- status: draft 
