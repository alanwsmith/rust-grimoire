#![allow(warnings)]
use axum::{response::Html, routing::get, Router};
use notify::RecursiveMode;
use notify::Watcher;
use notify_debouncer_mini::new_debouncer;
use notify_debouncer_mini::DebounceEventResult;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
// use tracing::{debug, error, info, span, warn, Level};

pub struct Site {
    pages: BTreeMap<String, Page>,
    input_dir: PathBuf,
    output_dir: PathBuf,
    // the valid extensions are to prevent tmp files
    // that end in e.g. `~`` from triggering
    valid_extension: Vec<String>,
}

pub struct Page {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let site = Site {
        pages: BTreeMap::new(),
        // needs to be full paths for now. the paths that 
        // come back from the watch are full regardless of
        // if this is set to relative path and I'm not sure
        // how to parse that out
        input_dir: PathBuf::from("/Users/alan/workshop/rust-playground.alanwsmith.com/site/web_server_with_hot_reload/_content"),
        output_dir: PathBuf::from("/Users/alan/workshop/rust-playground.alanwsmith.com/site/web_server_with_hot_reload/_site"),
        valid_extension: vec!["html".to_string(), "md".to_string(), "neo".to_string()],
    };
    tokio::spawn(async {
        let _ = watch_files(site);
    });
    let _ = run_web_server().await;
    Ok(())
}

async fn run_web_server() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("- Starting web server");
    let site_dir = "_site";
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();
    let app = Router::new()
        .nest_service("/", ServeDir::new(Path::new(site_dir)))
        .layer(livereload);
    let mut watcher = notify::recommended_watcher(move |_| reloader.reload())?;
    watcher.watch(Path::new(site_dir), notify::RecursiveMode::Recursive)?;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3443").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn watch_files(site: Site) -> notify::Result<()> {
    println!("- Starting file watcher");
    let (tx, rx) = std::sync::mpsc::channel();
    let mut debouncer = new_debouncer(Duration::from_millis(100), tx)?;
    debouncer
        .watcher()
        .watch(&site.output_dir, RecursiveMode::Recursive)?;
    for result in rx {
        let mut update_paths: BTreeSet<PathBuf> = BTreeSet::new();
        match result {
            Ok(events) => events.iter().for_each(|event| {
                let mut add_it = false;
                match event.path.extension() {
                    Some(p) => {
                        if site
                            .valid_extension
                            .contains(&p.to_string_lossy().to_string())
                        {
                            add_it = true;
                        }
                        ()
                    }
                    None => (),
                }
                if add_it {
                    update_paths.insert(event.path.clone());
                }
                ()
            }),
            Err(_) => {}
        }
        dbg!(update_paths);
    }
    Ok(())
}
