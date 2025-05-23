#[derive(Debug)]
pub enum Widget {
    Alfa(Vec<String>),
}

#[cfg(test)]
mod test {

    use crate::add_value_to_vec_in_enum::Widget;

    #[test]
    pub fn testit() {
        let mut w = Widget::Alfa(vec!["asdf".to_string()]);
        match w {
            Widget::Alfa(ref mut x) => {
                x.push("bravo".to_string());
                dbg!(x);
            }
        }
        // dbg!(w.push("asdf".to_string()));
        dbg!(w);
        assert_eq!(1, 1);
    }
}
