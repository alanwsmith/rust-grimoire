#![allow(warnings)]
use serde::ser::{
    Serialize,
    SerializeStruct,
    Serializer,
};

use serde::{
    Deserialize,
    Serialize,
};

// This is a test to write a serializer to
// allow methods to be used when creating
// a dataset. Staring from the example here:
// https://serde.rs/impl-serialize.html

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn alfa(&self) -> u8 {
        100
    }
}

impl Serialize for Color {
    fn serialize<S>(
        &self, serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer
            .serialize_struct("Color", 3)?;
        state.serialize_field("r", &self.r)?;
        state.serialize_field("g", &self.g)?;
        state.serialize_field("b", &self.b)?;
        state.serialize_field("x", &self.alfa())?;
        state.end()
    }
}

fn main() {
    let c = Color { r: 3, g: 4, b: 5 };

    dbg!(serde_yaml::to_string(&c));
}
