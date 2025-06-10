use grimoire::last_position::*;

#[test]
fn empty_string() {
    let text = "";
    let left = last_position(text).unwrap();
    let right = (0, 0);
    assert_eq!(left, right);
}

#[test]
fn solo_one_character() {
    let text = "a";
    let left = last_position(text).unwrap();
    let right = (0, 1);
    assert_eq!(left, right);
}

#[test]
fn two_characters_on_a_line() {
    let text = "ab";
    let left = last_position(text).unwrap();
    let right = (0, 2);
    assert_eq!(left, right);
}

#[test]
fn character_on_second_line() {
    let text = "a\nb";
    let left = last_position(text).unwrap();
    let right = (1, 1);
    assert_eq!(left, right);
}

#[test]
fn after_empty_lines() {
    let text = "\n\na b c";
    let left = last_position(text).unwrap();
    let right = (2, 5);
    assert_eq!(left, right);
}

#[test]
fn ending_empty_line() {
    let text = "abc\n";
    let left = last_position(text).unwrap();
    let right = (1, 0);
    assert_eq!(left, right);
}

#[test]
fn ending_empty_line_after_second() {
    let text = "abc\n\n";
    let left = last_position(text).unwrap();
    let right = (2, 0);
    assert_eq!(left, right);
}
