use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use tempfile::NamedTempFile;

#[derive(Debug, Serialize, Deserialize)]
pub struct Move {
    distance: u8,
}

fn main() -> Result<()> {
    let a = Move { distance: 10 };

    // Serialize a to a JSON string.
    let a_json_string = serde_json::to_string(&a)?;

    // Create a file inside of `std::env::temp_dir()`.
    let mut file1 = NamedTempFile::new()?;
    let mut file2 = file1.reopen()?;
    file1.write_all(a_json_string.as_bytes())?;

    // Read the test data using the second handle.
    let mut buf = String::new();
    file2.read_to_string(&mut buf)?;

    let b: Move = serde_json::from_str(&buf)?;

    // Print the JSON string, original Move struct, and deserialized Move struct.
    println!(
        "JSON string: {}\nOriginal Move: {:?}\nDeserialized Move: {:?}",
        buf, a, b
    );

    Ok(())
}
