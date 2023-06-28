// use miette::Severity::Error;
use core::fmt::Error;
use core::time::Duration;
use miette::{IntoDiagnostic, Result};
use std::path::PathBuf;
use watchexec::{
    action::Action,
    action::Outcome,
    config::{InitConfig, RuntimeConfig},
    //handler::PrintDebug,
    Watchexec,
};
use watchexec_events::filekind::FileEventKind;
use watchexec_events::filekind::ModifyKind;
use watchexec_events::Tag;

// NOTE: This was an attempt to deboune using the
// built in throttle, but it hangs onto all the
// events (which makes sence when I think about it)
// instead of just sending the last one (which is
// what I thought would happen). Will look for
// other options to debouce, but for now just
// gonna let things fly

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting process");
    let mut init = InitConfig::default();
    //    init.on_error(PrintDebug(std::io::stderr()));

    let mut runtime = RuntimeConfig::default();
    runtime.pathset(["/Users/alan/Desktop"]);
    runtime.action_throttle(Duration::new(1, 1000));

    // let conf = YourConfigFormat::load_from_file("watchexec.conf").await?;
    // conf.apply(&mut runtime);
    //
    //

    let we = Watchexec::new(init, runtime.clone())?;
    let w = we.clone();

    let c = runtime.clone();
    runtime.on_action(move |action: Action| {
        // let mut c = c.clone();
        // let w = w.clone();
        async move {
            for event in action.events.iter() {
                dbg!(event);
                // dbg!("delta");

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

                // if event.paths().any(|(p, _)| p.ends_with("/watchexec.conf")) {
                //     // let conf = YourConfigFormat::load_from_file("watchexec.conf").await?;
                //     // conf.apply(&mut c);
                //     // w.reconfigure(c.clone());
                //     // tada! self-reconfiguring watchexec on config file change!
                //     break;
                // }
            }

            action.outcome(Outcome::DoNothing);

            // action.outcome(Outcome::if_running(
            //     Outcome::DoNothing,
            //     Outcome::both(Outcome::Clear, Outcome::Start),
            // ));

            Ok::<(), Error>(())
        }
    });

    we.reconfigure(runtime);
    we.main().await.into_diagnostic()?.unwrap();
    // we.main().await;
    Ok(())
}

fn do_something(file_path: PathBuf) {
    // dbg!("ping");
    // dbg!(file_path);
}
