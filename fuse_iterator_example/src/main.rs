fn main() {
  let items = vec![
    "alfa", "bravo", "charlie", "", "delta", "echo",
  ];

  let refined: Vec<_> = items
    .iter()
    .map(
      |item| {
        if *item != "" { Some(item) } else { None }
      },
    )
    .fuse()
    .collect();

  dbg!(refined);
}
