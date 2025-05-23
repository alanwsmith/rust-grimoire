// NOTE: If you update this code, update it in
// id: 2riafnof as well
//

pub fn convert<'a>(vec_of_tuples: Vec<(&'a str, &'a str)>) -> Vec<&str> {
    vec_of_tuples.iter().map(|x| x.1).collect::<Vec<&str>>()
}

// And here's a test the confirms the functionality

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_convert() {
        let source: Vec<(&str, &str)> = vec![("alfa", "bravo"), ("charlie", "delta")];
        let expected: Vec<&str> = vec!["bravo", "delta"];
        assert_eq!(convert(source), expected);
    }
}

// NOTE: I've added the `::<>`` turbofish to `.collect()`` in
// the main funciton. Sometimes it's necessary, sometimes it's
// not. In this case the code would work without it like:
//
// vec_of_tuples.iter().map(|x| x.1).collect()
//
// I also added the type definition to `source`` and `expected``
// might not be necessary. That code would look like:
//
// let source = vec![("alfa", "bravo"), ("charlie", "delta")];
// let expected = vec!["bravo", "delta"];
//
// I'm not sure at what points those would be required and
// what points it's not so I defaulted to including them
// in the example
