use core::fmt::Error;
use miette::Result;
use std::path::PathBuf;
use std::time::Duration;
use watchexec::action::Action;
use watchexec::action::Outcome;
use watchexec::config::InitConfig;
use watchexec::config::RuntimeConfig;
use watchexec::Watchexec;
use watchexec_signals::Signal;

#[derive(Debug)]
struct Alfa {}

impl Alfa {
    pub fn update(&mut self, value: String) {
        dbg!(value);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut a = Alfa {};
    println!("Starting process");
    let init = InitConfig::default();
    let mut runtime = RuntimeConfig::default();
    runtime.pathset(["/Users/alan/Desktop"]);
    runtime.action_throttle(Duration::new(0, 150000));

    runtime.on_action(move |action: Action| {
        // let local_a = &mut a;
        // dbg!("hasdf");

        let _ = &a.update("HERE".to_string());

        async move {
            // // a.update("asdf".to_string());
            // // a.bravo = Some("asdf".to_string());
            // let mut stop: bool = false;
            // let mut paths: Vec<PathBuf> = vec![];
            // action.events.iter().for_each(|event| {
            //     event.signals().for_each(|sig| match sig {
            //         Signal::Interrupt => {
            //             println!("Caught Interrupt: Stopping");
            //             stop = true;
            //         }
            //         _ => {}
            //     });
            //     event
            //         .paths()
            //         .for_each(|path| paths.push(path.0.to_path_buf()));
            // });
            // paths.dedup();
            // paths
            //     .iter()
            //     .for_each(|path| println!("Change at {:?}", path));
            // if stop {
            //     action.outcome(Outcome::Exit);
            //     // } else {
            //     //     action.outcome(Outcome::if_running(
            //     //         Outcome::DoNothing,
            //     //         Outcome::both(Outcome::Clear, Outcome::Start),
            //     //     ));
            // }
            Ok::<(), Error>(())
        }
    });

    // runtime.on_action(move |action: Action| async move {
    //     // let local_a = &a;
    //     // a.update("asdf".to_string());
    //     // a.bravo = Some("asdf".to_string());
    //     let mut stop: bool = false;
    //     let mut paths: Vec<PathBuf> = vec![];
    //     action.events.iter().for_each(|event| {
    //         event.signals().for_each(|sig| match sig {
    //             Signal::Interrupt => {
    //                 println!("Caught Interrupt: Stopping");
    //                 stop = true;
    //             }
    //             _ => {}
    //         });
    //         event
    //             .paths()
    //             .for_each(|path| paths.push(path.0.to_path_buf()));
    //     });
    //     paths.dedup();
    //     paths
    //         .iter()
    //         .for_each(|path| println!("Change at {:?}", path));
    //     if stop {
    //         action.outcome(Outcome::Exit);
    //         // } else {
    //         //     action.outcome(Outcome::if_running(
    //         //         Outcome::DoNothing,
    //         //         Outcome::both(Outcome::Clear, Outcome::Start),
    //         //     ));
    //     }
    //     Ok::<(), Error>(())
    // });

    let we = Watchexec::new(init, runtime)?;
    we.main().await.unwrap().unwrap();
    Ok(())
}
