#![allow(unused)]
use anyhow::{Result, anyhow};
use grimoire_project::{
  cacher::Cacher, logger::Logger,
};
use redb::{Database, TableDefinition};
use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use tracing::{Level, event, instrument};
use tracing_subscriber::prelude::*;

struct Pages<'a> {
  catcher: Cacher<'a>,
  pages: Vec<Page>,
}

impl Pages<'_> {
  pub fn new() -> Result<Pages<'static>> {
    Ok(Pages {
      catcher: Cacher::new(&PathBuf::from(
        "storage.redb",
      ))?,
      pages: vec![],
    })
  }

  pub fn fetch_new_html(&mut self) -> Result<()> {
    for page in self.pages.iter_mut() {
      if let None = page.html {
        if let Ok(html) = fetch_file(&page.url) {
          &self.catcher.save(&page.url, &html);
          page.html = Some(Html::parse_document(&html));
        }
      }
    }
    Ok(())
  }

  pub fn load_cache(&mut self) -> Result<()> {
    event!(Level::INFO, "Loading from cache");
    for page in self.pages.iter_mut() {
      match self.catcher.get(&page.url) {
        Ok(cache) => {
          if let Some(html) = cache {
            page.html =
              Some(Html::parse_document(&html))
          }
        }
        Err(_) => {}
      }
    }
    Ok(())
  }

  pub fn load_input(&mut self) -> Result<()> {
    let content = fs::read_to_string("input.txt")?;
    for line in content.lines() {
      if line.starts_with("http") {
        self.pages.push(Page {
          url: line.trim().to_string(),
          html: None,
        })
      }
    }
    Ok(())
  }

  pub fn fetch_files(&mut self) -> Result<()> {
    for url in self.urls()?.iter() {
      event!(Level::INFO, "Fetching: {}", &url);
      let mut page = Page {
        url: url.to_string(),
        html: None,
      };
      let stored_data = self.catcher.get(&url);
      if let Err(e) = stored_data {}

      // if let Ok(v) = self.catcher.get(&url) {
      //   if let Some(_) = v {
      //     println!("Already have: {}", &url);
      //     continue;
      //   }
      // }

      //self.fetch_file(&url)?;
      //self.pages.push(page);
    }
    Ok(())
  }

  // fn make_links(&self) -> Result<Vec<String>> {
  //   let lines = self
  //     .urls()?
  //     .iter()
  //     .filter(|url| self.catcher.get(&url).is_ok())
  //     .filter(|url| {
  //       self.catcher.get(&url).unwrap().is_some()
  //     })
  //     .filter_map(|url| {
  //       let html =
  //         self.catcher.get(&url).unwrap().unwrap();
  //       get_title(&html)
  //     })
  //     .collect();
  //   Ok(lines)
  // }

  fn urls(&self) -> Result<Vec<String>> {
    let content = fs::read_to_string("input.txt")?;
    Ok(
      content
        .lines()
        .filter_map(|line| {
          if line.starts_with("http") {
            Some(line.to_string())
          } else {
            None
          }
        })
        .collect(),
    )
  }

  pub fn write_lines(&self) -> Result<()> {
    event!(Level::INFO, "Writing lines");
    let output = self
      .pages
      .iter()
      .map(|page| page.item())
      .collect::<Vec<String>>()
      .join("\n");
    fs::write("output.txt", output)?;
    Ok(())
  }
}

struct Page {
  url: String,
  html: Option<Html>,
}

impl Page {
  pub fn link(&self) -> String {
    match &self.html {
      None => {
        format!("[[{}|{}]]", self.url, self.url)
      }
      Some(html) => match get_title(&html) {
        Some(title) => {
          let txt =
            title.replace("  ", " ").replace("|", "~");
          format!("[[{}|{}]]", txt.trim(), self.url)
        }
        None => {
          format!("[[{}|{}]]", self.url, self.url)
        }
      },
    }
  }

  pub fn item(&self) -> String {
    if let Some(html) = &self.html {
      match get_description(html) {
        Some(desc) => {
          format!(
            "- {}\n\n{}",
            self.link(),
            desc
              .lines()
              .map(|line| format!("{}\n\n", line))
              .collect::<Vec<_>>()
              .join("")
          )
        }
        None => {
          format!("- {}\n", self.link())
        }
      }
    } else {
      "".to_string()
    }
  }
}

fn main() -> Result<()> {
  let _logger = Logger::new("INFO");
  event!(Level::INFO, "Starting");
  let mut pages = Pages::new()?;
  pages.load_input();
  pages.load_cache();
  // pages.fetch_new_html();
  pages.write_lines();

  // pages.fetch_files()?;

  //let links = pages.make_links()?;
  //dbg!(links);
  Ok(())
}

pub fn fetch_file(url: &str) -> Result<String> {
  event!(Level::INFO, "Fetching URL: {}", &url);
  let response = get(url)?;
  if response.status() == 200 {
    Ok(response.text()?)
  } else {
    Err(anyhow!("Could not get web page"))
  }
}

// fn init_logger() -> Result<()> {
//   let layer = tracing_subscriber::fmt::Layer::default()
//     .with_ansi(false)
//     .with_writer(std::io::stdout)
//     .compact();
//   let subscriber =
//     tracing_subscriber::Registry::default().with(layer);
//   tracing::subscriber::set_global_default(subscriber)?;
//   event!(Level::INFO, "Logger Initialized");
//   Ok(())
// }

fn get_title(document: &Html) -> Option<String> {
  let selector = Selector::parse("title").unwrap();
  let mut found = document.select(&selector);
  if let Some(title_tag) = found.next() {
    let title_text =
      title_tag.text().collect::<Vec<_>>().join("");
    Some(title_text)
  } else {
    None
  }
}

fn get_description(document: &Html) -> Option<&str> {
  if let Some(text) = get_og_description(document) {
    return Some(text);
  }
  get_meta_description(document)
}

fn get_meta_description(
  document: &Html
) -> Option<&str> {
  let selector =
    Selector::parse(r#"meta[name="description"]"#)
      .unwrap();
  let mut found = document.select(&selector);
  match found.next() {
    Some(tag) => tag.value().attr("content"),
    None => None,
  }
}

fn get_og_description(document: &Html) -> Option<&str> {
  let selector = Selector::parse(
    r#"meta[property="og:description"]"#,
  )
  .unwrap();
  let mut found = document.select(&selector);
  match found.next() {
    Some(tag) => tag.value().attr("content"),
    None => None,
  }
}

// fn load_pages(urls: &Vec<String>) -> Result<Vec<Page>> {
//     event!(Level::INFO, "Loading pages");
//     let pages = urls.iter().map(|url| {
//         dbg!(&url);
//         match fetch_url(url) {
//             Ok(html) => Page::WithHtml { url: url.to_string(), html: html},
//             Err(_) => Page::NoHtml { url: url.to_string()}
//         }
//     }).collect();
//     Ok(pages)
// }

// fn fetch_url(url: &str) -> Result<String> {
//   event!(Level::INFO, "Fetching URL: {}", &url);
//   let response = get(url)?;
//   if response.status() == 200 {
//     let html = response.text()?;
//     Ok(html)
//   } else {
//     Err(anyhow!("Could not fetch page"))
//   }
// }

// fn get_title(html: &str) -> Option<String> {
//     let document = Html::parse_document(html);
//     let selector = Selector::parse("title").unwrap();
//     let mut found = document.select(&selector);
//     if let Some(title_tag) = found.next() {
//       let title_text = title_tag.text().collect::<Vec<_>>().join("");
//       Some(title_text)
//     } else {
//       None
//     }
// }

// fn load_urls() -> Result<Vec<String>> {
//     event!(Level::INFO, "Loading URLs");
//     let content = fs::read_to_string("input.txt")?;
//     let urls = content.lines().filter_map(|line|
//         if
//         line.starts_with("http") {
//             Some(line.to_string())
//         } else { None}
//     ).collect();
//     Ok(urls)
// }

// fn save_item(
//   db: &Database,
//   key: &str,
//   value: &str,
// ) -> Result<()> {
//   let write_txn = db.begin_write()?;
//   {
//     let mut table = write_txn.open_table(TABLE)?;
//     table.insert(key, value)?;
//   }
//   write_txn.commit()?;
//   Ok(())
// }
