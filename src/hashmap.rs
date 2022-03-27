use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::token::{Types, UnWrappable};


pub struct HashMap<K, V> {
    index_array: Vec<Vec<(K, V)>>,
}

impl<K, V> HashMap<K, V>
where
K : Hash + Eq,
{
    pub fn new() -> Self {
        HashMap {
            index_array: Vec::new(),
        }
    }

    pub fn set(&mut self, key: K, value: V) {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        if self.index_array.len() == 0 {
            self.index_array.push(vec![(key, value)]);
            return;
        }

        let index = (hash % self.index_array.len() as u64) as usize;

        self.index_array[index].push((key, value));
    }

    pub fn delete(&mut self, key: K) {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        if self.index_array.len() == 0 {
            self.index_array.remove(0);
            return;
        }

        let index = (hash % self.index_array.len() as u64) as usize;

        let bucket = &mut self.index_array[index];

        if let Some(index) = bucket.iter().position(|(k, _)|{*k == key}) {
            bucket.remove(index);
        }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        if self.index_array.len() == 0 {
            return None;
        }

        let index = (hash % self.index_array.len() as u64) as usize;

        for (k, v) in &self.index_array[index] {
            if key == *k {
                return Some(v);
            }
        }
        None
    }
}
