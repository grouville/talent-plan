use serde::{Deserialize, Serialize};
use serde_json::{from_slice, to_vec, Result};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Move {
    distance: u8,
}

fn main() -> Result<()> {
    let a = Move { distance: 10 };

    // Serialize a to a JSON-formatted Vec<u8>.
    let serialized = to_vec(&a)?;

    // Deserialize b from the serialized Vec<u8>.
    let b: Move = from_slice(&serialized)?;

    println!("a: {:?}, b: {:?}", a, b);

    assert_eq!(a, b);

    Ok(())
}
