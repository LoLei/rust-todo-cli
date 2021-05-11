use crate::RtcError;
use pickledb::PickleDb;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Struct to hold a database implementation and some metadata
pub struct DatabaseWrapper<T: DbDriver> {
    pub driver: T,
    key_total_creations: String,
}

impl<T: DbDriver> DatabaseWrapper<T> {
    pub fn key_total_creations(&self) -> &String {
        &self.key_total_creations
    }

    pub fn new(db_driver: T) -> DatabaseWrapper<T> {
        DatabaseWrapper {
            driver: db_driver,
            key_total_creations: String::from("total_creations"),
        }
    }
    // This struct could also have the crud functions as methods
}

/// Implementations of this trait have the ability to perform certain CRUD operations
/// on a database system of their choice.
pub trait DbDriver {
    fn get<V: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<V>;
    fn get_all<V: for<'de> Deserialize<'de>>(&self) -> Vec<V>;
    fn set<V: Serialize>(&mut self, key: &str, value: &V) -> Result<(), Box<dyn Error>>;
    fn del(&mut self, key: &str) -> Result<bool, Box<dyn Error>>;
    fn n_keys(&self) -> usize;
    fn error_if_db_empty(&self, msg: &str) -> Result<(), RtcError>;
}

impl DbDriver for PickleDb {
    fn get<V: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<V> {
        self.get::<V>(key)
    }

    fn get_all<V: for<'de> Deserialize<'de>>(&self) -> Vec<V> {
        self.iter()
            // Could also use .filter and .is_some or filter_map to map Options to values,
            // and get rid of Nones
            // https://stackoverflow.com/a/30590082/4644044
            .flat_map(|kv| kv.get_value::<V>())
            .collect()
    }

    fn set<V: Serialize>(&mut self, key: &str, value: &V) -> Result<(), Box<dyn Error>> {
        match self.set(key, value) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn del(&mut self, key: &str) -> Result<bool, Box<dyn Error>> {
        match self.rem(key) {
            Ok(x) => Ok(x),
            Err(e) => Err(Box::new(e)),
        }
    }

    fn n_keys(&self) -> usize {
        self.total_keys()
    }

    fn error_if_db_empty(&self, msg: &str) -> Result<(), RtcError> {
        if self.n_keys() < 2 {
            Err(RtcError::NoItems(String::from(msg)))
        } else {
            Ok(())
        }
    }
}
