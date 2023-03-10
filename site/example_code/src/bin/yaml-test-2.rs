#![allow(warnings)]
// use anyhow::Result;
use serde::{
    Deserialize,
    Serialize,
};
use serde_yaml::Value as YAMLValue;
use std::collections::HashMap;

#[derive(
    Serialize, Deserialize, PartialEq, Debug,
)]
struct Widget {
    alfa: i32,
    bravo: String,
    charlie: Option<String>,
    delta: HashMap<String, i32>,
}

fn main() {
    let mut w = Widget {
        alfa: 3,
        bravo: String::from("quick fox"),
        charlie: Some(String::from("brown dog")),
        delta: HashMap::new(),
    };
    &w.delta.insert("HERE".to_string(), 7);

    dbg!(serde_yaml::to_string(&w));
}

// #[derive(Debug)]
// struct Example {
//     data: Option<YAMLValue>,
// }

// fn main() {
//     let text = r#"
// ---
// alfa: here"#;

// let data: Option<YAMLValue> =
//     serde_yaml::from_str(text).unwrap();

// println!("{}", text);

// let data = get_data(text);
// }

// fn get_data(text: &str) -> Result {

//}

// let mut example = Example { data: None };

// match get_yaml(&mut example) {
//     Ok(()) => println!("Loaded YAML"),
//     Err(e) => println!(
//         "Error loading YAML: {}",
//         e
//     ),
// }

// // dbg!(example.data.unwrap().get("alfa"
// ). // unwrap().as_str().unwrap());
// println!(
//     "{}",
//     example
//         .data
//         .unwrap()
//         .get("alfa")
//         .unwrap()
//         .as_str()
//         .unwrap()
// );

// println!("{}",
// example.data.unwrap().get("alfa").
// unwrap()); println!("{}",
// example.data.unwrap());
//
//

// fn get_yaml(
//     example: &mut Example,
// ) -> Result<()> {
//     example.data = serde_yaml::from_str(
//         r#"---
// alfa: here
// "#,
//     )?;
//     Ok(())
// }
