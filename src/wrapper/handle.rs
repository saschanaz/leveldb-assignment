use leveldb_sys::*;
use std::ffi::CString;
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::ptr::{self, NonNull};

use thiserror::Error;

use super::options::LevelDBOptions;
use super::read_options::LevelDBReadOptions;
use super::value::LevelDBValue;
use super::write_options::LevelDBWriteOptions;
use super::iterator::LevelDBIterator;

#[derive(Error, Debug)]
pub enum LevelDBError {
    #[error("Failed to open LevelDB")]
    FailedToOpen(String),
    #[error("Invalid DB name")]
    InvalidString,
}

pub struct LevelDBHandle {
    db_ptr: NonNull<leveldb_t>,
}

impl LevelDBHandle {
    pub fn open(name: &Path) -> Result<LevelDBHandle, LevelDBError> {
        let options = LevelDBOptions::new();

        let mut err = ptr::null_mut();

        let name = name
            .as_os_str()
            .to_str()
            .ok_or(LevelDBError::InvalidString)?;
        let name = CString::new(name).map_err(|_| LevelDBError::InvalidString)?;

        let (db_ptr, err_ptr) = unsafe {
            let db_ptr = leveldb_open(options.deref(), name.as_ptr(), &mut err);

            (db_ptr, err)
        };

        if err_ptr != ptr::null_mut() {
            return unsafe { Err(LevelDBError::FailedToOpen(format!("{}", *err_ptr))) };
        }

        Ok(Self {
            db_ptr: NonNull::new(db_ptr).expect("DB ptr should not be null when err_ptr is null"),
        })
    }

    pub fn put(&self, key: &[u8], data: &[u8]) -> Result<(), LevelDBError> {
        let options = LevelDBWriteOptions::new();
        let mut err = ptr::null_mut();
        unsafe {
            leveldb_put(
                self.db_ptr.as_ptr(),
                options.deref(),
                key.as_ptr() as _,
                key.len(),
                data.as_ptr() as _,
                data.len(),
                &mut err,
            )
        };

        if err != ptr::null_mut() {
            return unsafe { Err(LevelDBError::FailedToOpen(format!("{}", *err))) };
        }

        Ok(())
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<LevelDBValue>, LevelDBError> {
        let options = LevelDBReadOptions::new();
        let mut value_len = 0usize;
        let mut err = ptr::null_mut();
        let value = unsafe {
            leveldb_get(
                self.db_ptr.as_ptr(),
                options.deref(),
                key.as_ptr() as _,
                key.len(),
                &mut value_len,
                &mut err,
            )
        };

        if err != ptr::null_mut() {
            return unsafe { Err(LevelDBError::FailedToOpen(format!("{}", *err))) };
        }

        if value == ptr::null_mut() {
            // "Not found"
            return Ok(None);
        }

        let value = LevelDBValue::new(unsafe { NonNull::new_unchecked(value) }, value_len);
        Ok(Some(value))
    }

    pub fn iter(&self) -> LevelDBIterator {
        LevelDBIterator::new(self)
    }
}

impl Drop for LevelDBHandle {
    fn drop(&mut self) {
        unsafe { leveldb_close(self.db_ptr.as_ptr()) };
    }
}

impl Deref for LevelDBHandle {
    type Target = leveldb_t;

    fn deref(&self) -> &Self::Target {
        unsafe { self.db_ptr.as_ref() }
    }
}

impl DerefMut for LevelDBHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.db_ptr.as_mut() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn basic_open() -> Result<(), LevelDBError> {
        let dir = TempDir::new("basic_open").expect("Making tempdir");
        LevelDBHandle::open(dir.path())?;
        Ok(())
    }

    #[test]
    fn basic_put_get() -> Result<(), LevelDBError> {
        let dir = TempDir::new("basic_put_get").expect("Making tempdir");
        let handle = LevelDBHandle::open(dir.path())?;
        handle.put("foo".as_bytes(), "bar".as_bytes())?;
        let data = handle
            .get("foo".as_bytes())?
            .expect("We should have the data");
        assert_eq!(
            std::str::from_utf8(&*data).expect("data should be in utf8"),
            "bar"
        );
        Ok(())
    }

    #[test]
    fn basic_iterator() -> Result<(), LevelDBError> {
        let dir = TempDir::new("basic_put_get").expect("Making tempdir");
        let handle = LevelDBHandle::open(dir.path())?;
        handle.put("foo".as_bytes(), "bar".as_bytes())?;
        handle.put("foo2".as_bytes(), "bar2".as_bytes())?;

        let vec: Vec<&[u8]> = handle.iter().collect();

        let data = vec[0];
        assert_eq!(
            std::str::from_utf8(&*data).expect("data should be in utf8"),
            "bar"
        );

        let data = vec[1];
        assert_eq!(
            std::str::from_utf8(&*data).expect("data should be in utf8"),
            "bar2"
        );

        Ok(())
    }
}
