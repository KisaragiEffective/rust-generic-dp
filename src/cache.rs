use std::collections::HashMap;
use std::hash::Hash;

pub trait CachePolicy<K, V> {
    fn get(&self, k: &K) -> Option<&V>;
    fn set(&mut self, k: K, v: V) -> ();
}

pub struct CacheAll<K, V> {
    inner: HashMap<K, V>
}

impl<K: Eq + Hash, V> CachePolicy<K, V> for CacheAll<K, V> {
    fn get(&self, k: &K) -> Option<&V> {
        self.inner.get(k)
    }

    fn set(&mut self, k: K, v: V) -> () {
        self.inner.insert(k, v);
    }
}

pub struct NoCache;

impl<K, V> CachePolicy<K, V> for NoCache {
    fn get(&self, k: &K) -> Option<&V> {
        None
    }

    fn set(&mut self, k: K, v: V) -> () {
    }
}