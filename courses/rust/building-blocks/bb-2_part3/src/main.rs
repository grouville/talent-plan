use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::{Seek, SeekFrom, Write};
use tempfile::tempfile;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Move {
    distance: u8,
}

fn main() -> Result<()> {
    let moves: Vec<Move> = (1..=1000).map(|d| Move { distance: d as u8 }).collect();

    let mut temp_file = tempfile()?;

    for mov in &moves {
        let bson_data = bson::to_bson(&mov)?;
        let serialized = bson::ser::to_vec(&bson_data)?;
        temp_file.write_all(&serialized)?;
    }

    temp_file.sync_all()?;
    temp_file.seek(SeekFrom::Start(0))?;

    let mut deserialized_moves: Vec<Move> = Vec::new();
    while let Ok(bson_data) = bson::de::from_reader::<_, bson::Bson>(&mut temp_file) {
        let mov: Move = bson::from_bson(bson_data)?;
        deserialized_moves.push(mov);
    }

    assert_eq!(moves, deserialized_moves);

    println!("Successfully serialized and deserialized 1000 moves using BSON to/from a tempfile.");

    Ok(())
}
