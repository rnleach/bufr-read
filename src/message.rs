use crate::error::CodesError;
use eccodes_sys::{
    codes_get_error_message, codes_handle, codes_handle_delete, codes_set_long, CODES_SUCCESS,
};
use libc;
use std::ffi::CStr;

/// A single message from within a data source containing bufr data.
pub struct Message {
    handle: *mut codes_handle,
}

impl Message {
    // Create a new message.
    pub(crate) fn new(handle: *mut codes_handle) -> Result<Self, CodesError> {
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
