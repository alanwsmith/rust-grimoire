#[derive(Debug, PartialEq)]
struct Widget {
    alfa: String,
    bravo: Thing,
}

#[derive(Debug, PartialEq)]
enum Thing {
    Bravo { charlie: String },
}

fn main() {
    let widget_under_test = Widget {
        alfa: "arch".to_string(),
        bravo: Thing::Bravo {
            charlie: "delta".to_string(),
        },
    };

    assert_eq!(
        Widget {
            alfa: "arch".to_string(),
            bravo: Thing::Bravo {
                charlie: "delta".to_string(),
            },
        },
        widget_under_test,
    );

    println!("Tests passed");
}
