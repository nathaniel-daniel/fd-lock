use std::io::{self, Error};

use super::{RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug)]
pub struct RwLock<T> {
    pub(crate) _inner: T,
}

impl<T> RwLock<T> {
    #[inline]
    pub fn new(_inner: T) -> Self {
        panic!("target unsupported")
    }

    #[inline]
    pub fn write(&mut self) -> io::Result<RwLockWriteGuard<'_, T>> {
        panic!("target unsupported")
    }

    #[inline]
    pub fn try_write(&mut self) -> Result<RwLockWriteGuard<'_, T>, Error> {
        panic!("target unsupported")
    }

    #[inline]
    pub fn read(&self) -> io::Result<RwLockReadGuard<'_, T>> {
        panic!("target unsupported")
    }

    #[inline]
    pub fn try_read(&self) -> Result<RwLockReadGuard<'_, T>, Error> {
        panic!("target unsupported")
    }

    #[inline]
    pub fn into_inner(self) -> T
    where
        T: Sized,
    {
        panic!("target unsupported")
    }
}
