#![allow(clippy::module_name_repetitions)]

/*
use std::ops::Deref;
pub trait SmartPointerBackedCachePolicy<SPK: Deref<Target=K>, K, SPV: Deref<Target=V>, V> {
    fn get_smart_pointer(&self, k: SPK) -> Option<SPV>;
    fn set_by_smart_pointer(&self, k: SPK, v: SPV);
}

 */
use std::collections::HashMap;
use std::hash::Hash;



pub trait ArbitraryScopeCachePolicy<K, V>: for<'a> ScopedCachePolicy<'a, K, V> {
}

impl <ASCP: for<'a> ScopedCachePolicy<'a, K, V>, K, V> ArbitraryScopeCachePolicy<K, V> for ASCP {}

pub trait ScopedCachePolicy<'a, K, V> {
    fn get(&'a self, k: &K) -> Option<&'a V>;
    fn set(&'a mut self, k: K, v: V);
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

impl<'a, K: Eq + Hash, V> ScopedCachePolicy<'a, K, V> for CacheAll<K, V> {
    fn get(&'a self, k: &K) -> Option<&'a V> {
        self.inner.get(k)
    }

    fn set(&'a mut self, k: K, v: V) {
        self.inner.insert(k, v);
    }
}

pub struct NoCache;

impl<'a, K, V> ScopedCachePolicy<'a, K, V> for NoCache {
    fn get(&self, _k: &K) -> Option<&V> {
        None
    }

    fn set(&mut self, _k: K, _v: V) {
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

impl<'a, V: Clone + 'a> ScopedCachePolicy<'a, usize, V> for CacheVec<V> {
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
pub struct CacheArray<T, const N: usize>([Option<T>; N]);

impl <T: Clone, const L: usize> CacheArray<T, L> {
    pub fn new() -> Self {
        let temp_vec: Vec<Option<T>> = vec![None; L];
        let temp_slice = temp_vec.as_slice();
        let coerced_slice: &[Option<T>; L] = temp_slice.try_into().unwrap();
        let cloned_slice = coerced_slice.clone();
        Self(cloned_slice)
    }
}

impl <T: Clone, const N: usize> Default for CacheArray<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl <'a, V, const N: usize> ScopedCachePolicy<'a, usize, V> for CacheArray<V, N> {
    fn get(&self, k: &usize) -> Option<&V> {
        self.0.get(*k).and_then(Option::as_ref)
    }

    fn set(&mut self, k: usize, v: V) {
        self.0[k] = Some(v);
    }
}
