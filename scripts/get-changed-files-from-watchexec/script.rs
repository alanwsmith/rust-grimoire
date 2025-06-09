#!/usr/bin/env cargo -q -Zscript

---
[dependencies]
anyhow = "1.0.98"
clearscreen = "4.0.1"
itertools = "0.14.0"
tokio = { version = "1.45.1"}
watchexec = "8.0.1"
watchexec-events = "6.0.0"
watchexec-signals = "5.0.0"
---


use anyhow::Result;
use itertools::Itertools;
use std::sync::Arc;
use std::path::PathBuf;
use watchexec::command::{Command, Program, Shell};
use watchexec::{Id, Watchexec};
use watchexec_events::{Event, Tag};
use watchexec_events::filekind::*;
use watchexec_signals::Signal;

#[tokio::main]
async fn main() -> Result<()> {
    watch_for_changes().await?;
    Ok(())
}

async fn watch_for_changes() -> Result<()> {
    let wx = Watchexec::default();
    wx.config.pathset(vec!["."]);
    wx.config.on_action(move |mut action| {
        if action.signals().any(|sig| sig == Signal::Interrupt) {
            action.quit(); // Required for Ctrl+c to work
        } else {
            for path in get_paths(&action.events).iter() {
                clearscreen::clear().unwrap();
                let job = action.get_or_create_job(Id::default(), || make_command(&path));
                job.restart();
            }
        }
        action
    });
    println!("Watching files for changes");
    let _ = wx.main().await?;
    Ok(())
}

fn make_command(path: &PathBuf) -> Arc<Command> {
    Arc::new(
        Command{
            program: Program::Shell {
                shell: Shell::new("bash"),
                command: format!("cat {}", path.display()),
                args: vec![] // args for bash, not your command
            },
            options: Default::default()
        }
    )
}

fn get_paths(events: &Arc<[Event]>) -> Vec<PathBuf> {
    events
        .iter()
        .filter(|event| {
            event
                .tags
                .iter()
                .find(|tag| {
                    if let Tag::FileEventKind(kind) = &tag {
                        if let FileEventKind::Modify(mod_kind) = kind {
                            if let ModifyKind::Data(change) = mod_kind {
                                if let DataChange::Content = change {
                                    return true;
                                }
                            }
                        }
                    };
                    false
                })
                .is_some()
        })
        .filter_map(|event| {
            event.tags.iter().find_map(|tag| {
                if let Tag::Path { path, .. } = tag {
                    for component in path.components() {
                        if let std::path::Component::Normal(part) = component {
                            if part.display().to_string().starts_with(".") {
                                return None
                            }
                        }
                    };
                    if let Some(file_name_path) = path.file_name() {
                        let file_name = file_name_path.display().to_string();
                        if file_name.ends_with("~") {
                            return None;
                        }
                    };
                    Some(path.to_path_buf())
                } else {
                    None
                }
            })
        })
        .unique()
        .collect()
}

