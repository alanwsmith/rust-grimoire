#[derive(Debug)]
enum Alfa {
    Widget(Thing),
}

#[derive(Debug)]
struct Thing {
    text: String,
}

impl Alfa {
    fn update_text(&mut self) -> &Self {
        match self {
            Alfa::Widget(target) => {
                target.text = "werwerew".to_string();
            }
        }
        self
    }
}

fn main() {
    let mut thing = Alfa::Widget(Thing {
        text: "here".to_string(),
    });
    thing.update_text();
    dbg!(thing);
}
