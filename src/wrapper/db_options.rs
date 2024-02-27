use std::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use leveldb_sys::*;

pub struct LevelDBOptions {
    options: NonNull<leveldb_options_t>,
}

impl LevelDBOptions {
    pub fn new() -> Self {
        let options = unsafe { leveldb_options_create() };

        unsafe { leveldb_options_set_create_if_missing(options, true as u8) };

        Self {
            options: NonNull::new(options).expect("Options pointer should always be non-null"),
        }
    }
}

impl Drop for LevelDBOptions {
    fn drop(&mut self) {
        unsafe { leveldb_options_destroy(self.options.as_ptr()) };
    }
}

impl Deref for LevelDBOptions {
    type Target = leveldb_options_t;

    fn deref(&self) -> &Self::Target {
        unsafe { self.options.as_ref() }
    }
}
