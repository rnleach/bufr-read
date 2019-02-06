use crate::error::BufrErr;
use eccodes_sys::{
    codes_get_double, codes_get_error_message, codes_get_long, codes_handle, codes_handle_delete,
    codes_set_long, CODES_SUCCESS,
};
use libc;
use std::{
    borrow::Cow,
    ffi::{CStr, CString},
};

/// A single message from within a data source containing bufr data.
pub struct Message {
    handle: *mut codes_handle,
}

impl Message {
    /// Get an iterator over the keys available in this message.
    pub fn keys(&self) -> Result<impl Iterator<Item = Cow<str>>, BufrErr> {
        keys::KeysIterator::new(self)
    }

    /// Retrieve a long value from the message.
    pub fn long<K: Into<Vec<u8>>>(&self, key: K) -> Result<i64, BufrErr> {
        let key: CString = CString::new(key)?;
        let mut val = 0i64;

        unsafe {
            codes_check!(codes_get_long(self.handle, key.as_ptr(), &mut val))?;
        }

        Ok(val)
    }

    /// Retrieve a double value from the message.
    pub fn double<K: Into<Vec<u8>>>(&self, key: K) -> Result<f64, BufrErr> {
        let key: CString = CString::new(key)?;
        let mut val = 0f64;

        unsafe {
            codes_check!(codes_get_double(self.handle, key.as_ptr(), &mut val))?;
        }

        Ok(val)
    }

    // Create a new message.
    pub(crate) fn new(handle: *mut codes_handle) -> Result<Self, BufrErr> {
        unsafe {
            codes_check!(codes_set_long(
                handle,
                "unpack\0".as_ptr() as *const libc::c_char,
                1
            ))?;
        }

        Ok(Message { handle })
    }
}

impl Drop for Message {
    fn drop(&mut self) {
        unsafe {
            let err_code = codes_handle_delete(self.handle);
            if CODES_SUCCESS != err_code {
                let msg: &CStr = &CStr::from_ptr(codes_get_error_message(err_code));
                eprintln!("{}", msg.to_string_lossy());

                panic!("Error while deleting message handle.");
            }
        }
    }
}

pub mod keys;
