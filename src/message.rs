use crate::error::BufrErr;
use eccodes_sys::{
    codes_get_double, codes_get_double_array, codes_get_error_message, codes_get_long,
    codes_get_size, codes_handle, codes_handle_delete, codes_set_long, CODES_MISSING_DOUBLE,
    CODES_MISSING_LONG, CODES_SUCCESS,
};
use libc;
use optional::{none, some, Optioned};
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
    pub fn long<K: Into<Vec<u8>>>(&self, key: K) -> Result<Optioned<i64>, BufrErr> {
        let key: CString = CString::new(key)?;
        let mut val = 0i64;

        unsafe {
            codes_check!(codes_get_long(self.handle, key.as_ptr(), &mut val))?;
        }

        if val == CODES_MISSING_LONG {
            Ok(none())
        } else {
            Ok(some(val))
        }
    }

    /// Retrieve a double value from the message.
    pub fn double<K: Into<Vec<u8>>>(&self, key: K) -> Result<Optioned<f64>, BufrErr> {
        let key: CString = CString::new(key)?;
        let mut val = 0f64;

        unsafe {
            codes_check!(codes_get_double(self.handle, key.as_ptr(), &mut val))?;
        }

        if val == CODES_MISSING_DOUBLE {
            Ok(none())
        } else {
            Ok(some(val))
        }
    }

    /// Retrieve an array of double values from the message.
    pub fn double_array<K: Into<Vec<u8>>>(&self, key: K) -> Result<Vec<Optioned<f64>>, BufrErr> {
        let key: CString = CString::new(key)?;
        let mut num_vals: u64 = 0;
        let mut vals: Vec<Optioned<f64>>;

        unsafe {
            codes_check!(codes_get_size(self.handle, key.as_ptr(), &mut num_vals))?;
            vals = vec![some(0.0); num_vals as usize];
            codes_check!(codes_get_double_array(
                self.handle,
                key.as_ptr(),
                vals.as_mut_ptr() as *mut f64, // !! treating Optioned<f64> as f64!
                &mut num_vals
            ))?;
            vals.set_len(num_vals as usize);
        }

        for val in vals.iter_mut() {
            if (*val).unpack() == CODES_MISSING_DOUBLE {
                *val = none();
            }
        }

        Ok(vals)
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
