use std::collections::HashMap;

// This works, there's a more comlicated version in the
// neopolitan parser as well taht deal with some
// nom stuff passing to a second funciton.

#[derive(Debug)]
struct Widget {
    alfa: HashMap<String, String>,
}

fn main() {
    let mut w = Widget {
        alfa: HashMap::new(),
    };
    w.update();
    dbg!(w);
}

impl Widget {
    pub fn update(&mut self) {
        self.alfa
            .insert("A".to_string(), "B".to_string());
    }
}
