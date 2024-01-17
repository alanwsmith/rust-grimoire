#![allow(warnings)]
use axum::{response::Html, routing::get, Router};
use notify::RecursiveMode;
use notify::Watcher;
use notify_debouncer_full::{new_debouncer, notify::*, DebounceEventResult};
// use notify_debouncer_mini::new_debouncer;
// use notify_debouncer_mini::DebounceEventResult;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::fs::remove_file;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use tower_livereload::Reloader;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
pub struct Site {
    pages: BTreeSet<PathBuf>,
    input_dir: PathBuf,
    output_dir: PathBuf,
    valid_extensions: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("- Starting main");
    let site = Site {
        pages: BTreeSet::new(),
        input_dir: PathBuf::from("/Users/alan/workshop/rust-playground.alanwsmith.com/site/web_server_with_hot_reload/_content"),
        output_dir: PathBuf::from("/Users/alan/workshop/rust-playground.alanwsmith.com/site/web_server_with_hot_reload/_site"),
        valid_extensions: vec!["neo".to_string(), "html".to_string()],
    };
    let _ = run_web_server(site).await;
    Ok(())
}

async fn run_web_server(site: Site) -> Result<()> {
    println!("- Starting web server");
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();
    let app = Router::new()
        .nest_service("/", ServeDir::new(Path::new(&site.output_dir)))
        .layer(livereload);
    tokio::spawn(async {
        let _ = watch_files(site, reloader);
    });
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3443").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn watch_files(mut site: Site, reloader: Reloader) -> notify::Result<()> {
    println!("- Loading initial files");
    let walker = WalkDir::new(&site.input_dir).into_iter();
    site.pages = walker
        .filter_map(|e| match e {
            Ok(d) => Some(d.into_path()),
            Err(err) => None,
        })
        .filter_map(|path| match path.extension() {
            Some(ext) => {
                if ext == "html" {
                    Some(path)
                } else {
                    None
                }
            }
            None => None,
        })
        .collect();

    println!("- Making initial queue");
    let queue: Vec<PathBuf> = site.pages.iter().map(|page| page.clone()).collect();
    process_queue(queue);

    Ok(())
}

fn process_queue(mut queue: Vec<PathBuf>) {
    while queue.len() > 0 {
        // do the work here
        dbg!(queue.pop());
    }
    dbg!(queue);
}

// println!("- Buiding initial site");
// build_full_site(&site, &reloader);
// // println!("- Buiding initila home page");
// // build_home_page(&site);
// println!("- Starting file watcher");
// let (tx, rx) = std::sync::mpsc::channel();
// let mut debouncer = new_debouncer(Duration::from_millis(100), tx)?;
// debouncer
//     .watcher()
//     .watch(&site.input_dir, RecursiveMode::Recursive)?;
// for result in rx {
//     let mut update_paths: BTreeSet<PathBuf> = BTreeSet::new();
//     match result {
//         Ok(events) => events.iter().for_each(|event| {
//             let mut add_it = false;
//             match event.path.extension() {
//                 Some(p) => {
//                     if site
//                         .valid_extension
//                         .contains(&p.to_string_lossy().to_string())
//                     {
//                         add_it = true;
//                     }
//                     ()
//                 }
//                 None => (),
//             }
//             if add_it {
//                 update_paths.insert(event.path.clone());
//             }
//             ()
//         }),
//         Err(_) => {}
//     }
//     update_paths.iter().for_each(|path| {
//         let output_rel_path = &path.strip_prefix(&site.input_dir).unwrap();
//         let mut output_path = site.output_dir.clone();
//         output_path.push(&output_rel_path);
//         if file_exists(&path) {
//             if &output_path != &site.home_page {
//                 // do work here to apply templates, etc...
//                 build_page(path, &output_path);
//                 // add the page to the list of site pages
//                 // then build the home page
//                 let mut site_path = PathBuf::from("/");
//                 site_path.push(&path.strip_prefix(&site.input_dir).unwrap().to_path_buf());
//                 site.pages
//                     .insert(path.display().to_string(), Page { site_path });
//                 build_home_page(&site);
//             } else {
//                 // the home page changed, rebuild it
//                 build_home_page(&site);
//             }
//         } else {
//             site.pages.remove(&path.display().to_string());
//             // remove the page from the output directory if
//             // it exists
//             if file_exists(&output_path) {
//                 let _ = remove_file(output_path);
//             }
//             build_home_page(&site);
//         }
//     });
// }
