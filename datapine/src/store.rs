use std::collections::HashMap;
use serde::{Serialize, Deserialize};


pub mod dump;
pub mod simmlarity_search;
#[cfg(test)]
pub mod tests;
pub mod action;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Store {
    inner: HashMap<String, Vec<f32>>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            inner: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, value: Vec<f32>) {
        self.inner.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&Vec<f32>> {
        self.inner.get(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<Vec<f32>> {
        self.inner.remove(key)
    }
}

pub use self::action::*;
pub use self::simmlarity_search::*;