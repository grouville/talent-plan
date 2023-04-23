use anyhow::Result;
use ron;
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Debug, Serialize, Deserialize)]
pub struct Move {
    distance: u8,
}

fn main() -> Result<()> {
    let a = Move { distance: 10 };

    // Serialize a to a RON string.
    let a_ron_string = ron::ser::to_string(&a)?;

    // Serialize a to a Vec<u8> buffer.
    let mut buffer = Vec::new();
    buffer.extend_from_slice(a_ron_string.as_bytes());

    // Deserialize a from the Vec<u8> buffer.
    let ron_str = str::from_utf8(&buffer)?;
    let b: Move = ron::de::from_str(ron_str)?;

    // Print the serialized string representation and the deserialized Move struct.
    println!(
        "Serialized RON string: {}\nDeserialized Move: {:?}",
        ron_str, b
    );

    Ok(())
}
