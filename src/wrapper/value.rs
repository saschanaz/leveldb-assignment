use std::{ffi::c_char, ops::Deref, ptr::NonNull};

use leveldb_sys::*;

pub struct LevelDBValue(NonNull<c_char>, usize);

impl LevelDBValue {
    pub fn new(value: NonNull<c_char>, len: usize) -> Self {
        Self(value, len)
    }
}

impl Drop for LevelDBValue {
    fn drop(&mut self) {
        unsafe { leveldb_free(self.0.as_ptr() as *mut _) };
    }
}

impl Deref for LevelDBValue {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.0.as_ptr() as *mut u8, self.1) }
    }
}
