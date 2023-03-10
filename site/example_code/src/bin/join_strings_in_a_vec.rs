fn main() {
    let mut widget: Vec<String> = vec![];
    widget.push("alfa".to_string());
    widget.push("bravo".to_string());
    widget.push("charlie".to_string());

    let output = widget.join("\n");

    println!("{}", output);
}
