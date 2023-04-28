// cargo add miette
// cargo add watchexec

// NOTES:
// - Copy and paste didn't work: got an error:
// `main` function is not allowed to be `async`
//
// - Example uses stubbed "YourConfigFormat"
// gotta figure out what's actually needed there
//
// - Bailing on this one for another example
// from the examples folder

use miette::{IntoDiagnostic, Result};
use watchexec::{
    action::{Action, Outcome},
    config::{InitConfig, RuntimeConfig},
    handler::{Handler as _, PrintDebug},
    Watchexec,
};

fn main() {
    watch_things();
}

#[tokio::main]
async fn watch_things() -> Result<()> {
    let mut init = InitConfig::default();
    init.on_error(PrintDebug(std::io::stderr()));

    let mut runtime = RuntimeConfig::default();
    runtime.pathset(["watchexec.conf"]);

    let conf = YourConfigFormat::load_from_file("watchexec.conf").await?;
    conf.apply(&mut runtime);

    let we = Watchexec::new(init, runtime.clone())?;
    let w = we.clone();

    let c = runtime.clone();
    runtime.on_action(move |action: Action| {
        let mut c = c.clone();
        let w = w.clone();
        async move {
            for event in action.events.iter() {
                if event.paths().any(|(p, _)| p.ends_with("/watchexec.conf")) {
                    let conf = YourConfigFormat::load_from_file("watchexec.conf").await?;

                    conf.apply(&mut c);
                    w.reconfigure(c.clone());
                    // tada! self-reconfiguring watchexec on config file change!

                    break;
                }
            }

            action.outcome(Outcome::if_running(
                Outcome::DoNothing,
                Outcome::both(Outcome::Clear, Outcome::Start),
            ));

            Ok(())
        }
    });

    we.reconfigure(runtime);
    we.main().await.into_diagnostic()?;
    Ok(())
}
