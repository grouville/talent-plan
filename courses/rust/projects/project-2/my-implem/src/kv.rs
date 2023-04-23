use crate::error::{MyError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

pub struct KvStore {
    store: HashMap<String, String>,
    file: BufWriter<File>,
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

        let reader = BufReader::new(&log_file);
        for line in reader.lines() {
            let cmd: Command = serde_json::from_str(&line?)?;
            match cmd {
                Command::Set { key, value } => {
                    store.insert(key, value);
                }
                Command::Remove { key } => {
                    store.remove(&key);
                }
            }
        }

        let file = BufWriter::new(log_file.try_clone()?);

        Ok(KvStore { store, file })
    }

    pub fn new() -> KvStore {
        let path = Path::new(".");
        KvStore::open(path).unwrap()
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let cmd = Command::Set {
            key: key.clone(),
            value: value.clone(),
        };
        let cmd_str = serde_json::to_string(&cmd)?;

        // Flush and sync the file before writing to it
        self.file.flush()?;
        self.file.get_ref().sync_all()?;

        writeln!(self.file, "{}", cmd_str)?;

        self.store.insert(key, value);
        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        Ok(self.store.get(&key).cloned())
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.store.remove(&key).is_some() {
            let cmd = Command::Remove { key: key.clone() };
            let cmd_str = serde_json::to_string(&cmd)?;

            // Flush and sync the file before writing to it
            self.file.flush()?;
            self.file.get_ref().sync_all()?;

            writeln!(self.file, "{}", cmd_str)?;

            Ok(())
        } else {
            Err(MyError::KeyNotFound)
        }
    }
}
