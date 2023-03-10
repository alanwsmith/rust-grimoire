use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug, PartialEq, Serialize, Deserialize,
)]
struct Top {
    alfa: Middle,
    bravo: Middle,
}

#[derive(
    Debug, PartialEq, Serialize, Deserialize,
)]
struct Middle {
    charlie: u64,
    delta: u64,
}

fn main() -> Result<(), serde_yaml::Error> {
    let m1 = Middle {
        charlie: 1,
        delta: 2,
    };
    let m2 = Middle {
        charlie: 1,
        delta: 2,
    };

    let top = Top {
        alfa: m1,
        bravo: m2,
    };

    let yaml = serde_yaml::to_string(&top)?;

    dbg!(yaml);

    // assert_eq!(yaml, "x: 1.0\ny: 2.0\n");

    //let deserialized_point: Point =
    //   serde_yaml::from_str(&yaml)?;
    // assert_eq!(point, deserialized_point);
    Ok(())
}
