use crate::error::{MyError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::Path;

pub struct KvStore {
    pub store: HashMap<String, u64>,
    writer: BufWriter<File>,
    reader: BufWriter<File>,
    offset: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl KvStore {
    pub fn open(path: &Path) -> Result<KvStore> {
        let dir = path.to_path_buf();
        let log_path = dir.join("kvs.log");

        let mut store = HashMap::new();

        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(&log_path)?;

        let mut reader = BufReader::new(&log_file);
        let mut offset = reader.seek(SeekFrom::Start(0))?;

        let mut deserializer = serde_json::Deserializer::from_reader(&mut reader);
        while let Some(cmd) = Command::deserialize(&mut deserializer).ok() {
            let cmd_bytes = serde_json::to_vec(&cmd)?;

            match cmd {
                Command::Set { key, value: _ } => {
                    store.insert(key, offset);
                }
                Command::Remove { key } => {
                    store.remove(&key);
                }
            }
            offset += cmd_bytes.len() as u64;
        }
        // println!("Offset: {:?}", store);

        let writer = BufWriter::new(log_file.try_clone()?);
        let reader = BufWriter::new(log_file.try_clone()?);

        Ok(KvStore {
            store,
            writer,
            reader,
            offset,
        })
    }

    pub fn new() -> KvStore {
        let path = Path::new(".");
        KvStore::open(path).unwrap()
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::Set {
            key: key.clone(),
            value,
        };
        let offset = self.offset;

        // Write the command to the file
        serde_json::to_writer(&mut self.writer, &cmd)?;
        self.writer.flush()?;

        let cmd_bytes = serde_json::to_vec(&cmd)?;

        // insert in the store
        self.store.insert(key, offset + cmd_bytes.len() as u64);

        // Get the len of the serialized command
        // println!("Offset_set: {}", self.offset);
        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        // println!("Offset_get_crap: {:?}", self.store);
        match self.store.get(&key) {
            Some(offset) => {
                // println!("Offset: {}", offset);

                let mut reader = BufReader::new(self.reader.get_ref());
                reader.seek(SeekFrom::Start(offset.clone()))?;
                let mut deserializer = serde_json::Deserializer::from_reader(reader);

                let cmd_result = Command::deserialize(&mut deserializer);

                match cmd_result {
                    Ok(cmd) => match cmd {
                        Command::Set { key: _, value } => Ok(Some(value)),
                        Command::Remove { key: _ } => Ok(None),
                    },
                    Err(_) => Err(MyError::KeyNotFound),
                }
            }
            None => Ok(None),
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.store.remove(&key).is_some() {
            let cmd = Command::Remove { key: key.clone() };

            serde_json::to_writer(&mut self.writer, &cmd)?;
            self.writer.flush()?;

            // Get the len of the serialized command
            let cmd_bytes = serde_json::to_vec(&cmd)?;
            self.offset += cmd_bytes.len() as u64; // Update the offset
            Ok(())
        } else {
            Err(MyError::KeyNotFound)
        }
    }
}
