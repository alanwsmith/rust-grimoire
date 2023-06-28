#[derive(Debug)]
enum Widget {
    Alfa,
    Bravo,
    Charlie,
}

fn main() {
    let widgets: Vec<Widget> = vec![
        Widget::Alfa,
        Widget::Alfa,
        Widget::Bravo,
        Widget::Charlie,
        Widget::Charlie,
    ];

    for item in widgets.iter() {
        match item {
            Widget::Alfa => {
                println!("a");
            }
            Widget::Bravo => {
                println!("b");
            }
            Widget::Charlie => {
                println!("c");
            }
        }
    }
}
