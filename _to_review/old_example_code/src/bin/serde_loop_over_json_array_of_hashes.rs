use serde_json::Value;

fn main() {
    let data_string = r#"
    { "thing": 
        [ 
            { "name": "alfa" },
            { "name": "bravo" },
            { "name": "charlie" }

        ]
    }"#;

    let data: Value =
        serde_json::from_str(data_string).unwrap();

    if let Some(items) = data["thing"].as_array() {
        for item in items {
            if let Some(values) = item.as_object() {
                if let Some(name) =
                    values["name"].as_str()
                {
                    println!("Name: {}", name);
                }
            }
        }
    }
}
