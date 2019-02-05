#![macro_use]

use eccodes_sys::codes_get_error_message;
use libc;
use std::{error::Error, ffi::CStr, fmt::Display};

#[derive(Debug)]
pub struct CodesError(libc::c_int);

impl Display for CodesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        unsafe {
            let msg: &CStr = &CStr::from_ptr(codes_get_error_message(self.0));
            write!(f, "{}", msg.to_string_lossy())
        }
    }
}

impl Error for CodesError {}

impl From<libc::c_int> for CodesError {
    fn from(val: libc::c_int) -> Self {
        CodesError(val)
    }
}

macro_rules! codes_check {
    ($code:expr) => {
        if $code != eccodes_sys::CODES_SUCCESS {
            Err(CodesError::from($code))
        } else {
            Ok(())
        }
    };
}
