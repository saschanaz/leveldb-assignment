use leveldb_sys::*;
use std::ffi::CString;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::ptr::{self, NonNull};

use thiserror::Error;

use super::db_options::LevelDBOptions;

#[derive(Error, Debug)]
pub enum LevelDBError {
    #[error("Failed to open LevelDB")]
    FailedToOpen(String),
    #[error("Invalid DB name")]
    InvalidDBName,
}

pub struct LevelDBHandle {
    db_ptr: NonNull<leveldb_t>,
}

impl LevelDBHandle {
    pub fn open(name: &PathBuf) -> Result<LevelDBHandle, LevelDBError> {
        let options = LevelDBOptions::new();

        let mut err = ptr::null_mut();

        let name = name
            .as_os_str()
            .to_str()
            .ok_or(LevelDBError::InvalidDBName)?;
        let name = CString::new(name).map_err(|_| LevelDBError::InvalidDBName)?;

        let (db_ptr, err_ptr) = unsafe {
            let db_ptr = leveldb_open(options.deref(), name.as_ptr(), &mut err);

            (db_ptr, err)
        };

        if err_ptr == ptr::null_mut() {
            Ok(Self {
                db_ptr: NonNull::new(db_ptr)
                    .expect("DB ptr should not be null when err_ptr is null"),
            })
        } else {
            unsafe { Err(LevelDBError::FailedToOpen(format!("{}", *err_ptr))) }
        }
    }
}

impl Drop for LevelDBHandle {
    fn drop(&mut self) {
        unsafe { leveldb_close(self.db_ptr.as_ptr()) };
    }
}

#[test]
fn basic_open() -> Result<(), LevelDBError> {
    LevelDBHandle::open(&std::env::temp_dir())?;
    Ok(())
}
