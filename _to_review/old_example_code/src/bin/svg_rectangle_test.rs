// use svg::node::element::path::Data;
// use svg::node::element::Path;
use svg::node::element::Rectangle;
use svg::Document;

fn main() {
    let rect = Rectangle::new()
        .set("x", "10")
        .set("y", "10")
        .set("width", "10")
        .set("height", "10")
        .set("fill", "#373");

    let document = Document::new()
        .set("viewBox", (0, 0, 70, 70))
        .add(rect);

    svg::save("svg-output-test-3.svg", &document)
        .unwrap();
}
