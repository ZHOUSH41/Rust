#![deny(missing_docs)]
//! A simple key/value store

use std::collections::HashMap;
use std::result;
use std::io;
use std::path::PathBuf;

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
    store: HashMap<String, String>,
}

/// Custom error type
#[derive(Debug)]
pub enum Error {
    /// Io error
    Io(io::Error)
}

/// Custom result type
pub type Result<T> = result::Result<T, Error>;
/// method of KvStore
impl KvStore {
    /// Creates a `KvStore`
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        Ok(KvStore {
            store: HashMap::new(),
        })
    }

    /// Sets the value of a string key to a string.
    /// Returns `Ok` if success,
    /// Returns `Err` if failure.
    pub fn set(&mut self, key: String, value: String) -> Result<()>{
        self.store.insert(key, value);
        panic!()
    }

    /// Gets the string value of a given string key.
    /// Returns `Ok(Some)` when getting a existent key,
    /// Returns `Ok(None)` when getting a non-existent key,
    /// Retruns `Err` when error
    pub fn get(&self, key: String) -> Result<Option<String>> {
        self.store.get(&key).cloned();
        panic!()
    }

    /// Return `Ok(Some)` previously stored value when removing a existent key,
    /// return `Ok(None)` when removing a non-existent key,
    /// return `Err` when error
    pub fn remove(&mut self, key: String) -> Result<Option<String>>{
        self.store.remove(&key);
        panic!()
    }
}
