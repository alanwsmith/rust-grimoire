use core::fmt::Error;
use core::time::Duration;
use miette::Result;
use std::path::PathBuf;
use watchexec::{
    action::Action,
    action::Outcome,
    config::{InitConfig, RuntimeConfig},
    Watchexec,
};
use watchexec_events::filekind::FileEventKind;
use watchexec_events::filekind::ModifyKind;
use watchexec_events::Tag;

// This is doing the debouncing by adding event
// paths to a vec and then deduping it that way.
// It's doing what I expect though I wouldn't be
// surprised to find a more native way.

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting process");
    let init = InitConfig::default();
    let mut runtime = RuntimeConfig::default();
    runtime.pathset(["/Users/alan/Desktop"]);
    // I think this is the time in nanoseconds that the
    // debounc takes, but it starts from the leading edge
    // so things can trigger if they happen right at
    // the end of the time. So, like it's not really
    // a debounce but something like it.
    runtime.action_throttle(Duration::new(0, 100000));
    let we = Watchexec::new(init, runtime.clone())?;
    runtime.on_action(move |action: Action| {
        async move {
            let mut events: Vec<PathBuf> = vec![];
            for event in action.events.iter() {
                let mut trigger: bool = false;
                let mut file_path: Option<PathBuf> = None;
                event.tags.iter().for_each(|tag| match tag {
                    Tag::Path { path, .. } => {
                        file_path = Some(path.to_path_buf());
                        events.push(file_path.clone().unwrap());
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
            }
            events.dedup();
            events.iter().for_each(|p| do_something(p.to_path_buf()));
            // Not sure if this is necessary or not
            // TBD on that
            action.outcome(Outcome::DoNothing);
            Ok::<(), Error>(())
        }
    });
    // This can probably be setup differently so you don't
    // have to reload with reconfigure.
    we.reconfigure(runtime).unwrap();
    we.main().await.unwrap().unwrap();
    Ok(())
}

fn do_something(file_path: PathBuf) {
    dbg!(file_path);
}
