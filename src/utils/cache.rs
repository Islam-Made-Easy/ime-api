// use std::collections::HashMap;
// use std::time::{Duration, Instant};

// struct Cache<K, V> {
//     map: HashMap<K, (Instant, V)>,
//     ttl: Duration,
// }

// impl<K, V> Cache<K, V>
// where
//     K: Eq + std::hash::Hash,
// {
//     fn new(ttl: Duration) -> Self {
//         Cache {
//             map: HashMap::new(),
//             ttl,
//         }
//     }

//     fn get(&self, key: &K) -> Option<&V> {
//         if let Some((instant, value)) = self.map.get(key) {
//             if instant.elapsed() < self.ttl {
//                 return Some(value);
//             }
//         }
//         None
//     }

//     fn insert(&mut self, key: K, value: V) {
//         self.map.insert(key, (Instant::now(), value));
//     }

//     fn remove(&mut self, key: &K) -> Option<V> {
//         if let Some((_, value)) = self.map.remove(key) {
//             return Some(value);
//         }
//         None
//     }

//     fn clear(&mut self) {
//         self.map.clear();
//     }
// }
