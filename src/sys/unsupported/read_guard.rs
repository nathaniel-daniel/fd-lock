use std::ops;

use super::RwLock;

#[derive(Debug)]
pub struct RwLockReadGuard<'lock, T> {
    _lock: &'lock RwLock<T>,
}

impl<T> ops::Deref for RwLockReadGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        panic!("target unsupported")
    }
}

impl<T> Drop for RwLockReadGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        panic!("target unsupported")
    }
}
