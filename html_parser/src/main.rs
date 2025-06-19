use anyhow::Result;
use scraper::{Html, Selector};

fn main() -> Result<()> {
  let document = load_html();

  let title = get_title(&document)
    .or(Some("N/A".to_string()))
    .unwrap();
  println!("Title: {}", title);

  let meta_description =
    get_meta_description(&document)
      .or(Some("N/A"))
      .unwrap();
  println!("Meta Description: {}", meta_description);

  let og_description = get_og_description(&document)
    .or(Some("N/A"))
    .unwrap();
  println!("Meta Description: {}", og_description);

  let description =
    get_description(&document).or(Some("N/A")).unwrap();
  println!("Description: {}", description);

  Ok(())
}

fn load_html() -> Html {
  let html = r#"
    <!DOCTYPE html>
    <meta charset="utf-8">
    <meta name="description" content="This is the meta description">
    <meta property="og:description" content="This is the og description">
    <title>Hello, world!</title>
    <h1 class="foo">Hello, <i>world!</i></h1>
"#;
  Html::parse_document(html)
}

fn get_title(document: &Html) -> Option<String> {
  let selector = Selector::parse("title").unwrap();
  let mut found = document.select(&selector);
  match found.next() {
    Some(tag) => {
      Some(tag.text().collect::<Vec<_>>().join(""))
    }
    None => None,
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
