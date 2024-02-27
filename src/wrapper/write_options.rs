use std::{ops::Deref, ptr::NonNull};

use leveldb_sys::*;

pub struct LevelDBWriteOptions {
    options: NonNull<leveldb_writeoptions_t>,
}

impl LevelDBWriteOptions {
    pub fn new() -> Self {
        let options = unsafe { leveldb_writeoptions_create() };

        Self {
            options: NonNull::new(options).expect("Options pointer should always be non-null"),
        }
    }
}

impl Drop for LevelDBWriteOptions {
    fn drop(&mut self) {
        unsafe { leveldb_writeoptions_destroy(self.options.as_ptr()) };
    }
}

impl Deref for LevelDBWriteOptions {
    type Target = leveldb_writeoptions_t;

    fn deref(&self) -> &Self::Target {
        unsafe { self.options.as_ref() }
    }
}
