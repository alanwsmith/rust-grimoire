#![allow(warnings)]
use axum::{response::Html, routing::get, Router};
use notify::RecursiveMode;
use notify::Watcher;
use notify_debouncer_mini::new_debouncer;
use notify_debouncer_mini::DebounceEventResult;
use std::collections::BTreeMap;
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
}

pub struct Page {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let site = Site {
        pages: BTreeMap::new(),
        input_dir: PathBuf::from("_content"),
        output_dir: PathBuf::from("_site"),
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
    //     let (tx, rx) = std::sync::mpsc::channel();
    //     let mut debouncer = new_debouncer(Duration::from_millis(300), tx)?;
    //     debouncer.watcher().watch(RecursiveMode::Recursive)?;
    //     for result in rx {
    //         match result {
    //             Ok(events) => events.iter().for_each(|event| {
    //                 println!("- Hard coding home page output");
    //             }),
    //             Err(_) => {}
    //         }
    //     }
    Ok(())
}
