use rand::Rng;
use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

pub mod events;

#[derive(Debug)]
pub struct Cache<T>(Arc<RwLock<InnerCache<T>>>);

impl<T> Clone for Cache<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T> Cache<T> {
    pub fn new(val: T) -> Cache<T> {
        Self(Arc::new(RwLock::new(InnerCache { tag: rand::rng().random(), cache: val })))
    }

    pub fn read(&self) -> CacheReadGuard<'_, T> {
        CacheReadGuard(self.0.read().unwrap())
    }

    pub fn write(&self) -> CacheWriteGuard<'_, T> {
        CacheWriteGuard(self.0.write().unwrap())
    }

    pub fn rand_tag(&self) {
        let mut write = self.write();
        write.0.tag = rand::rng().random();
    }
}

struct InnerCache<T> {
    tag: [u8;5],
    cache: T,
}

impl<T: std::fmt::Debug> std::fmt::Debug for InnerCache<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("InnerCache").field(&self.cache).finish()
    }
}

pub struct CacheReadGuard<'a,T>(RwLockReadGuard<'a,InnerCache<T>>);
pub struct CacheWriteGuard<'a,T>(RwLockWriteGuard<'a,InnerCache<T>>);

impl<T> Deref for CacheReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0.cache
    }
}

impl<T> Deref for CacheWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0.cache
    }
}

impl<T> DerefMut for CacheWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.cache
    }
}

