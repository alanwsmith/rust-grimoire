#![allow(warnings)]
// use svg::node::element::Circle;
use svg::node::element::Mask;
use svg::node::element::Rectangle;
use svg::node::Node;
use svg::Document;

// This makes a mask and applies it to
// a rectangle. The parts of the mask
// that are white show up in the viewing
// rectangle. The parts that are black
// are tranparent

fn main() {
    let show_rect = Rectangle::new()
        .set("x", "0")
        .set("y", "0")
        .set("width", "400")
        .set("height", "400")
        .set("fill", "white");

    let hide_rect = Rectangle::new()
        .set("x", "100")
        .set("y", "50")
        .set("width", "200")
        .set("height", "100")
        .set("fill", "black");

    let mask = Mask::new()
        .set("id", "main-mask")
        .add(show_rect)
        .add(hide_rect);

    let view_rect = Rectangle::new()
        .set("x", "0")
        .set("y", "0")
        .set("width", "400")
        .set("height", "400")
        .set("fill", "green")
        .set("mask", "url(#main-mask");

    let document = Document::new()
        .set("viewBox", (0, 0, 400, 400))
        .add(mask)
        .add(view_rect);

    svg::save("svg-output-test-4.svg", &document)
        .unwrap();
}
