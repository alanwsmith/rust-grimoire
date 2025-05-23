use async_priority_channel as priority;
use std::path::PathBuf;
use tokio::sync::{mpsc, watch};
use watchexec::{
    event::{Event, Priority},
    fs,
};
use watchexec_events::filekind::FileEventKind;
use watchexec_events::filekind::ModifyKind;
use watchexec_events::Tag;

// Warning: If you do:
// touch a.txt && echo "asdf" > a.txt && rm a.txt
// instead of: create, modify, delete
// watchexec might see:
// create, delete, modify
// because of that, putting the check for if it's
// a file in the function that gets called.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let (ev_s, ev_r) = priority::bounded::<Event, Priority>(1024);
    let (er_s, _er_r) = mpsc::channel(64);
    let (wd_s, wd_r) = watch::channel(fs::WorkingData::default());

    let mut wkd = fs::WorkingData::default();
    wkd.pathset = vec!["/Users/alan/Desktop".into()];
    wd_s.send(wkd).unwrap();

    tokio::spawn(async move {
        while let Ok((event, _priority)) = ev_r.recv().await {
            let mut trigger: bool = false;
            let mut file_path: Option<PathBuf> = None;
            event.tags.iter().for_each(|tag| match tag {
                Tag::Path { path, .. } => {
                    file_path = Some(path.to_path_buf());
                }
                Tag::FileEventKind(event_kind) => match event_kind {
                    FileEventKind::Create(_) => {
                        trigger = true;
                    }
                    FileEventKind::Modify(modify_kind) => match modify_kind {
                        ModifyKind::Data(_) => {
                            trigger = true;
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            });
            if trigger {
                do_something(file_path.unwrap());
            }
        }
    });
    fs::worker(wd_r, er_s, ev_s).await?;
    Ok(())
}

fn do_something(file_path: PathBuf) {
    dbg!("############ FROM V7 #################");
    if file_path.is_file() {
        dbg!(file_path);
    }
}
