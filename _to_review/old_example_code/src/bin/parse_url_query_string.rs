use url::Url;

fn main() {
    let url = Url::parse("https://www.example.com/index.html?alfa=bravo&charlie=delta")
        .expect("Could not parse url");
    let query_pairs = url.query_pairs();
    query_pairs.for_each(|param| {
        println!("key: {}, value: {}", param.0, param.1);
        ()
    });
}


