use std::collections::{HashMap, BTreeMap};
use std::path::PathBuf;
use std::fs::{self, File, OpenOptions};
use crate::{KvsError, Result};
/// The `KvStore` stores string key/value pairs.
/// Key/value pairs are stored in a `Hashmap` in memory and not persisted to disk/
///
/// Example:
/// ``` rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
#[derive(Default)]
pub struct KvStore {
    store: HashMap<String, String>
}

/// method of KvStore
impl KvStore {
    /// Creates a `KvStore`
    pub fn new() -> KvStore {
        KvStore {
            store: HashMap::new(),
        }
    }

    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) -> Result<()>{
        self.store.insert(key, value);
        Ok(())
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> Result<Option<String>>{
        Ok(self.store.get(&key).cloned())
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) -> Result<()> {
        self.store.remove(&key);
        Ok(())
    }

    /// Opens a `KvStore` with the given path
    /// This will create a new directory if the given one does not exist.
    ///
    /// # Errors
    ///
    /// It propagates I/O or deserialization errors during the log replay.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let path = path.into();
        fs::create_dir_all(&path)?;
        Ok(KvStore {
            store: HashMap::new(),
        })
    }
}
