use rand::Rng;

fn main() {
    let items = vec!["alfa", "bravo", "charlie", "delta"];
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..items.len());
    let random_item = items[random_index];
    dbg!(random_item);
}

// cargo add rand
