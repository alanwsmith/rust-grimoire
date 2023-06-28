use core::fmt::Error;
use miette::Result;
use watchexec::action::Action;
use watchexec::action::Outcome;
use watchexec::config::InitConfig;
use watchexec::config::RuntimeConfig;
use watchexec::Watchexec;
use watchexec_signals::Signal;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Starting process");
    let init = InitConfig::default();
    let mut runtime = RuntimeConfig::default();
    runtime.on_action(move |action: Action| async move {
        let mut stop: bool = false;
        action.events.iter().for_each(|event| {
            event.signals().for_each(|sig| match sig {
                Signal::Interrupt => {
                    println!("Caught Interrupt: Stopping");
                    stop = true;
                }
                _ => {}
            });
        });
        action.outcome(Outcome::Exit);
        Ok::<(), Error>(())
    });
    let we = Watchexec::new(init, runtime)?;
    we.main().await.unwrap().unwrap();
    Ok(())
}
