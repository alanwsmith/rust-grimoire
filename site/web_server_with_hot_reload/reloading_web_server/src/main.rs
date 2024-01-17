use axum::Router;
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use notify_debouncer_mini::DebounceEventResult;
use std::collections::BTreeSet;
use std::fs;
use std::fs::remove_file;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;
use tower_livereload::Reloader;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Site {
    pages: BTreeSet<PathBuf>,
    input_dir: PathBuf,
    output_dir: PathBuf,
    valid_extensions: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

async fn run_web_server(site: Site) -> Result<(), Box<dyn std::error::Error>> {
    println!("- Starting web server");
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();
    let app = Router::new()
        .nest_service("/", ServeDir::new(Path::new(&site.output_dir)))
        .layer(livereload);
    tokio::spawn(async move {
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
            Err(_) => None,
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
    process_queue(queue, &site);
    let _ = &reloader.reload();

    // this feels like a hack, but it was what let me
    // get around problems with the move
    let second_site = site.clone();

    let mut debouncer = new_debouncer(
        Duration::from_millis(100),
        move |res: DebounceEventResult| match res {
            Ok(events) => {
                let queue: Vec<PathBuf> = events
                    .iter()
                    .filter_map(|event| match event.path.extension() {
                        Some(ext) => {
                            if second_site
                                .clone()
                                .valid_extensions
                                .contains(&ext.to_string_lossy().to_string())
                            {
                                Some(event.clone().path)
                            } else {
                                None
                            }
                        }
                        None => None,
                    })
                    .collect();
                process_queue(queue, &second_site);
                let _ = &reloader.reload();
            }
            Err(e) => println!("Error {:?}", e),
        },
    )
    .unwrap();

    debouncer
        .watcher()
        .watch(Path::new(&site.input_dir), RecursiveMode::Recursive)
        .unwrap();
    // need an endless loop to keep the debounder from
    // dropping itself
    loop {}
}

fn process_queue(mut queue: Vec<PathBuf>, site: &Site) {
    while queue.len() > 0 {
        match queue.pop() {
            Some(input_path) => {
                let rel_path = &input_path.strip_prefix(&site.input_dir).unwrap();
                dbg!(&rel_path);
                let mut output_path = site.output_dir.clone();
                output_path.push(rel_path);
                let _ = fs::copy(input_path, output_path);
            }
            None => (),
        }
    }
}
