use crate::error::{MyError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Seek, SeekFrom, Write};
use std::path::Path;

const REDUNDANT_COMMANDS_THRESHOLD: u64 = 100;

pub struct KvStore {
    pub store: HashMap<String, u64>,
    writer: BufWriter<File>,
    reader: BufReader<File>,
    offset: u64,
    generation: u64,
    dir: std::path::PathBuf,
    redundant_commands: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Set { key: String, value: String },
    Remove { key: String },
}

impl KvStore {
    pub fn open(path: &Path) -> Result<KvStore> {
        let dir = path.to_path_buf();

        // Find the log file with the highest generation number
        let regex = regex::Regex::new(r"^kvs-(\d+)\.log$")?;
        let highest_generation = std::fs::read_dir(&dir)?
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let path = entry.path();
                path.file_name()
                    .and_then(|f| f.to_str())
                    .and_then(|filename| regex.captures(filename))
                    .and_then(|captures| captures[1].parse::<u64>().ok())
            })
            .max()
            .unwrap_or(0);

        let log_path = dir.join(format!("kvs-{}.log", highest_generation));

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

        let writer = BufWriter::new(log_file.try_clone()?);
        let reader = BufReader::new(log_file.try_clone()?);

        Ok(KvStore {
            store,
            writer,
            reader,
            offset,
            generation: highest_generation,
            dir,
            redundant_commands: 0,
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
        let previous_offset = self.store.get(&key).cloned();

        // Write the command to the file
        serde_json::to_writer(&mut self.writer, &cmd)?;
        self.writer.flush()?;

        // add to hashmap
        self.store.insert(key, offset as u64);

        // compute new offset
        let cmd_bytes = serde_json::to_vec(&cmd)?;
        self.offset = offset + cmd_bytes.len() as u64;

        // Increase the redundant commands counter if the key already existed
        if previous_offset.is_some() {
            self.redundant_commands += 1;
        }

        // Call the compact method based on a heuristic, e.g., when the number of redundant commands
        // reaches a certain threshold
        if self.redundant_commands >= REDUNDANT_COMMANDS_THRESHOLD {
            self.compact()?;
        }

        Ok(())
    }

    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        match self.store.get(&key) {
            Some(offset) => {
                self.reader.seek(SeekFrom::Start(offset.clone()))?;
                let mut deserializer = serde_json::Deserializer::from_reader(&mut self.reader);

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

            // Increase the redundant commands counter
            self.redundant_commands += 1;

            // Call the compact method based on a heuristic, e.g., when the number of redundant commands
            // reaches a certain threshold
            if self.redundant_commands >= REDUNDANT_COMMANDS_THRESHOLD {
                self.compact()?;
            }

            Ok(())
        } else {
            Err(MyError::KeyNotFound)
        }
    }

    pub fn compact(&mut self) -> Result<()> {
        self.generation += 1;
        let new_log_path = self.dir.join(format!("kvs-{}.log", self.generation));

        let new_log_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&new_log_path)?;
        let mut new_writer = BufWriter::new(&new_log_file);

        let new_store = HashMap::new();
        let mut new_offset = 0;

        for (store_key, offset) in self.store.iter() {
            self.reader.seek(SeekFrom::Start(offset.clone()))?;
            let mut deserializer = serde_json::Deserializer::from_reader(&mut self.reader);
            let cmd_result = Command::deserialize(&mut deserializer)?;

            if let Command::Set { key: _, value } = cmd_result {
                serde_json::to_writer(
                    &mut new_writer,
                    &Command::Set {
                        key: store_key.clone(),
                        value: value.clone(),
                    },
                )?;
                let cmd_bytes = serde_json::to_vec(&Command::Set {
                    key: store_key.clone(),
                    value,
                })?;
                new_offset += cmd_bytes.len() as u64;
            } else {
                return Err(MyError::UnknownError);
            }
        }

        let new_log_file_reader = OpenOptions::new().read(true).open(&new_log_path)?;
        let new_log_file_writer = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&new_log_path)?;
        self.reader = BufReader::new(new_log_file_reader);
        self.writer = BufWriter::new(new_log_file_writer);

        self.store = new_store;
        self.offset = new_offset;
        self.redundant_commands = 0;
        let old_log_path = self.dir.join(format!("kvs-{}.log", self.generation - 1));
        std::fs::remove_file(&old_log_path)?;
        Ok(())
    }
}
