// Pretend that this is somebody else's crate, not a module.
mod other_crate {
    // Neither Serde nor the other crate provides Serialize and Deserialize
    // impls for this struct. Oh, and the fields are private.
    pub struct Duration {
        secs: i64,
        nanos: i32,
    }
    impl Duration {
        pub fn new(secs: i64, nanos: i32) -> Self {
            Duration {
                secs: secs,
                nanos: nanos,
            }
        }
        pub fn seconds(&self) -> i64 {
            self.secs
        }
        pub fn subsec_nanos(&self) -> i32 {
            self.nanos
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

use other_crate::Duration;
use serde::{
    Deserialize,
    Serialize,
};

// Provide getters for every private field of the remote struct. The getter must
// return either `T` or `&T` where `T` is the type of the field.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Duration")]
struct DurationDef {
    #[serde(getter = "Duration::seconds")]
    secs: i64,
    #[serde(getter = "Duration::subsec_nanos")]
    nanos: i32,
}

// Provide a conversion to construct the remote type.
impl From<DurationDef> for Duration {
    fn from(def: DurationDef) -> Duration {
        Duration::new(def.secs, def.nanos)
    }
}

#[derive(Serialize, Deserialize)]
struct Process {
    command_line: String,
    #[serde(with = "DurationDef")]
    wall_time: Duration,
}

fn main() {
    println!("--SDF");
}
