#![allow(warnings)]

// Seems there's not a way to do this directly.
// The answer is to use `match` or `if let`

enum Widget {
    Alfa { strings: Vec<String> },
}

fn main() {
    let wa = Widget::Alfa { strings: vec![] };

    // wa.strings.push("a".to_string()); // no field `strings` on type `Widget`

    // wa.Alfa.strings.push("a".to_string()); // no field `Alfa` on type `Widget`

    // wa.alfa.strings.push("a".to_string()); // no field `alfa` on type `Widget`

    // wa.Widget::Alfa.strings.push("a".to_string()); // expected one of `(`, `.`, `;`, `?`, `}`, or an operator, found `::`

    // wa["strings"].push("a".to_string()); // cannot index into a value of type `Widget`
}
