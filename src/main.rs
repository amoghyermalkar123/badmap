use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
mod in_memory;
mod hmp_parser;
mod token;

struct HashMap<K, V> {
    index_array: Vec<Vec<(K, V)>>,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    fn new() -> Self {
        HashMap {
            index_array: Vec::new(),
        }
    }

    fn set(&mut self, key: K, value: V) {
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

    fn delete(&mut self, key: K) {
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

    fn get(&self, key: K) -> Option<&V> {
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

fn main() {
    let mut hm = HashMap::<u64, String>::new();

    for i in 1..6 {
        hm.set(i, format!("value is {value}", value = i).to_string());
    }

    for i in 1..6 {
        let ans = hm.get(i);

        println!("answer is {:?}", ans.unwrap());
    }

    hm.delete(2);

    for i in 1..6 {
        let ans = hm.get(i);

        println!("answer is {:?}", ans.unwrap_or(&"none".to_string()));
    }
}
