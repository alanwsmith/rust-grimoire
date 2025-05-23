#![allow(warnings)]

// This is a way to process methods/funcitons
// and seralize the output via remote setup:
// https://serde.rs/remote-derive.html
// Not sure it's the best way to go about it.
// Also going to look at making a custom
// seralizer which seems more likely.
//
// Yeah, looks like making a custom seralizer
// (in example yaml-test-5.rs) is the way to
// go. Much less jumping through hoops.

mod other_crate {
    pub struct Widget {}
    impl Widget {
        pub fn new() -> Self {
            Widget {}
        }
        pub fn alfa(&self) -> i64 {
            123
        }
    }
}

//////////////////////////////////////////////////

use other_crate::Widget;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize)]
#[serde(remote = "Widget")]
struct WidgetDef {
    #[serde(getter = "Widget::alfa")]
    alfa: i64,
}

impl From<WidgetDef> for Widget {
    fn from(def: WidgetDef) -> Widget {
        Widget::new()
    }
}

#[derive(Serialize, Deserialize)]
struct Process {
    #[serde(with = "WidgetDef")]
    widget: Widget,
}

fn main() {
    println!("quick fox");
    let w = Widget {};
    let p = Process { widget: Widget {} };
    dbg!(serde_yaml::to_string(&p));
}
