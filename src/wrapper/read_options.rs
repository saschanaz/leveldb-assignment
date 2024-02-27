use std::{ops::Deref, ptr::NonNull};

use leveldb_sys::*;

pub struct LevelDBReadOptions {
    options: NonNull<leveldb_readoptions_t>,
}

impl LevelDBReadOptions {
    pub fn new() -> Self {
        let options = unsafe { leveldb_readoptions_create() };

        Self {
            options: NonNull::new(options).expect("Options pointer should always be non-null"),
        }
    }
}

impl Drop for LevelDBReadOptions {
    fn drop(&mut self) {
        unsafe { leveldb_readoptions_destroy(self.options.as_ptr()) };
    }
}

impl Deref for LevelDBReadOptions {
    type Target = leveldb_readoptions_t;

    fn deref(&self) -> &Self::Target {
        unsafe { self.options.as_ref() }
    }
}
