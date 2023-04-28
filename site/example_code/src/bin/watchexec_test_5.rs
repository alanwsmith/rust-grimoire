// This loads the convience wrapper, but I'm not
// sure how to use it after that. Going back
// to the other code example to see about
// adding debounce in the next version

use watchexec::config::InitConfig;
use watchexec::config::RuntimeConfig;
use watchexec::Watchexec;

#[tokio::main]
async fn main() {
    println!("Starting");
    let we = Watchexec::new(InitConfig::default(), RuntimeConfig::default()).unwrap();
    we.main();
    println!("Done");
}
