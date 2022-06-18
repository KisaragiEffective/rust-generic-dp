#![allow(clippy::module_name_repetitions)]

use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub trait CachePolicy<K, V> {
    fn get(&self, k: &K) -> Option<&V>;
    fn set(&mut self, k: K, v: V);
}

pub struct CacheAll<K, V> {
    inner: HashMap<K, V>
}

impl<K, V> CacheAll<K, V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new()
        }
    }
}
impl<K: Eq + Hash, V> CachePolicy<K, V> for CacheAll<K, V> {
    fn get(&self, k: &K) -> Option<&V> {
        self.inner.get(k)
    }

    fn set(&mut self, k: K, v: V) {
        self.inner.insert(k, v);
    }
}

pub struct NoCache;

impl<K, V> CachePolicy<K, V> for NoCache {
    fn get(&self, k: &K) -> Option<&V> {
        None
    }

    fn set(&mut self, k: K, v: V) {
    }
}

#[derive(Default, Debug)]
pub struct CacheVec<T>(Vec<Option<T>>);

impl <V> CacheVec<V> {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn new_with_capacity(initial_capacity: usize) -> Self {
        let vec = Vec::with_capacity(initial_capacity);
        Self(vec)
    }
}

impl <V: Clone> CachePolicy<usize, V> for CacheVec<V> {
    fn get(&self, k: &usize) -> Option<&V> {
        self.0.get(*k).and_then(Option::as_ref)
    }

    fn set(&mut self, k: usize, v: V) {
        let vec = &mut self.0;
        let len = vec.len();
        if k >= len {
            vec.resize(k + 1, None);
        }

        let len = vec.len();
        // println!("new length: {len}, assets: {k}");
        assert!(len >= k, "len = {len}, k = {k}");

        vec[k] = Some(v);
    }
}

#[derive(Debug)]
pub struct CacheArray<T: ?Sized, const N: usize>([Option<Rc<T>>; N]);

impl <T: Clone + ?Sized, const N: usize> CacheArray<T, N> {
    pub fn new() -> Self {
        CacheArray([None; N])
    }
}

impl <T, const N: usize> Default for CacheArray<T, N> {
    fn default() -> Self {
        Self([None; N])
    }
}

impl <V, const N: usize> CachePolicy<usize, V> for CacheArray<V, N> {
    fn get(&self, k: &usize) -> Option<&V> {
        self.0.get(*k)
            .and_then(Option::as_ref)
            .map(|ref_box| ref_box.clone())
            .map(|rc| rc.as_ref())
    }

    fn set(&mut self, k: usize, v: V) {
        self.0[k] = Some(Rc::new(v));
    }
}
