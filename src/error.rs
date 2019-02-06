#![macro_use]

use eccodes_sys::codes_get_error_message;
use libc;
use std::{error::Error, ffi::CStr, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufrErr {
    CodesError(libc::c_int),
    NullPtr,
}

impl Display for BufrErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            BufrErr::NullPtr => write!(f, "null pointer encountered."),
            BufrErr::CodesError(val) => unsafe {
                let msg: &CStr = &CStr::from_ptr(codes_get_error_message(val));
                write!(f, "{}", msg.to_string_lossy())
            },
        }
    }
}

impl Error for BufrErr {}

impl From<libc::c_int> for BufrErr {
    fn from(val: libc::c_int) -> Self {
        BufrErr::CodesError(val)
    }
}

macro_rules! codes_check {
    ($code:expr) => {
        if $code != eccodes_sys::CODES_SUCCESS {
            Err(crate::error::BufrErr::from($code))
        } else {
            Ok(())
        }
    };
}
