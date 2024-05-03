use std::collections::HashMap; 
use std::sync::{Arc, Mutex}; 

#[derive(Clone)]
pub struct ConcurrentHashMap<K, V> { 
  inner: Arc<Mutex<HashMap<K, V>>>, 
} 

impl<K, V: Clone> ConcurrentHashMap<K, V> 
where K: std::cmp::Eq + std::hash::Hash + Send + 'static, 
V: Send + 'static, {

  pub fn new() -> Self { 
    ConcurrentHashMap { 
      inner: Arc::new(Mutex::new(HashMap::new())), } 
    } 
    
    pub fn insert(&self, key: K, value: V){ 
    let mut inner = self.inner.lock().unwrap(); 
    inner.insert(key, value); } 
  
    pub fn get(&self, key: &K) -> Option<V> { 
    let inner = self.inner.lock().unwrap(); 
    inner.get(key).cloned() 
  } 
}