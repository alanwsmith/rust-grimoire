use uuid::Uuid;

fn main() {
    let id = Uuid::new_v4().simple().to_string();
    let token = id.get(0..6).unwrap();
    // let as_string = id.simple().to_string();
    //  get(0..8).unwrap();
    // dbg!(token);
    dbg!(token);
    // println!("{}", id);
    // println!("{}", id.simple().to_string().get(0..8).unwrap());
}
