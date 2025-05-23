use async_priority_channel as priority;
use tokio::sync::{mpsc, watch};
use watchexec::fs::{worker, WorkingData};

// This is from here:
// https://docs.rs/watchexec/latest/src/watchexec/fs.rs.html
// GOAL: Add coded to make it do something when files change.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (ev_s, _) = priority::bounded(1024);
    let (er_s, _) = mpsc::channel(64);
    let (wd_s, wd_r) = watch::channel(WorkingData::default());
    let mut wkd = WorkingData::default();
    wkd.pathset = vec![".".into()];
    wd_s.send(wkd)?;

    tokio::spawn(async move {
        while let Ok((event, priority)) = ev_r.recv().await {
            tracing::info!("event ({priority:?}): {event:?}");
        }
    });

    worker(wd_r, er_s, ev_s).await?;

    Ok(())
}
