use std::ops;

use super::RwLock;

#[derive(Debug)]
pub struct RwLockWriteGuard<'lock, T> {
    _lock: &'lock mut RwLock<T>,
}

impl<T> ops::Deref for RwLockWriteGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        panic!("target unsupported")
    }
}

impl<T> ops::DerefMut for RwLockWriteGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        panic!("target unsupported")
    }
}

impl<T> Drop for RwLockWriteGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        panic!("target unsupported")
    }
}
