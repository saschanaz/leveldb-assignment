use std::{marker::PhantomData, ops::Deref, ptr::NonNull};

use leveldb_sys::*;

use super::{read_options::LevelDBReadOptions, LevelDBHandle};

pub struct LevelDBIterator<'a> {
    iterator: NonNull<leveldb_iterator_t>,
    started: bool,
    phantom: PhantomData<&'a LevelDBHandle>,
}

impl<'a> LevelDBIterator<'a> {
    pub fn new(db: &'a LevelDBHandle) -> Self {
        let options = LevelDBReadOptions::new();
        let iterator =
            unsafe { leveldb_create_iterator(db.deref() as *const _ as *mut _, options.deref()) };
        Self {
            iterator: NonNull::new(iterator).expect("Iterator should be non-null"),
            started: false,
            phantom: PhantomData,
        }
    }
}

impl<'a> Drop for LevelDBIterator<'a> {
    fn drop(&mut self) {
        unsafe { leveldb_iter_destroy(self.iterator.as_ptr()) }
    }
}

impl<'a> std::iter::Iterator for LevelDBIterator<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let it = self.iterator.as_ptr();
        if !self.started {
            unsafe { leveldb_iter_seek_to_first(it) };
            self.started = true;
        } else {
            unsafe { leveldb_iter_next(it) };
        }

        let valid = unsafe { leveldb_iter_valid(it) } != 0;
        if !valid {
            return None;
        }

        let mut len = 0usize;
        let value = unsafe { leveldb_iter_value(it, &mut len) };

        let value: &[u8] = unsafe { std::slice::from_raw_parts(value as *mut u8, len) };
        Some(value)
    }
}
