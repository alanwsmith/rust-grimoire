use anyhow::Result;
use grimoire::last_position::*;

fn main() -> Result<()> {
    println!("{:?}", last_position("").unwrap());
    Ok(())
}
