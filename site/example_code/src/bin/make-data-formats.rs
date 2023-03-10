use serde_json::json;

fn main() {
    output_json();
}

fn output_json() -> String {
    println!("Making JSON");
    let text = String::from(
        r#"{
    "alfa": "apple", 
    "bravo": 3,
    "charlie": [
        "cherry", 5
    ],
    "delta": {
        "echo": "egg",
        "foxtrot": 7
    }
}"#,
    );

    json!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });

    text
}
