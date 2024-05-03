use std::collections::HashMap;
use std::sync::{Arc, RwLock};


pub struct ConcurrentHashMap<K, V> {
    inner: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V:Clone> ConcurrentHashMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Send + 'static,
    V: Send + 'static,
{
    pub fn new() -> Self {
        ConcurrentHashMap {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn insert(&self, key: K, value: V) {
        self.inner.write().unwrap().insert(key, value);
    }

    pub fn get(&self, key: &K) -> Option<V> {
        self.inner.read().unwrap().get(key).cloned()
    }
}