use crate::message::Message;
use eccodes_sys::{codes_handle, codes_handle_new_from_file, ProductKind, CODES_SUCCESS};
use libc;
use std::ffi::CString;

/// A file on the file system containing bufr messages.
pub struct BufrFile {
    src: *mut libc::FILE,
}

impl Drop for BufrFile {
    fn drop(&mut self) {
        if !self.src.is_null() {
            unsafe {
                libc::fclose(self.src);
            }
        }
    }
}

impl BufrFile {
    pub fn new(path: &str) -> Result<Self, std::io::Error> {
        const MODE: *const libc::c_char = "r\0".as_ptr() as *const libc::c_char;
        unsafe {
            let fname = CString::new(path)?;
            let src: *mut libc::FILE = libc::fopen(fname.as_ptr(), MODE);

            if src.is_null() {
                Err(std::io::Error::from(std::io::ErrorKind::Other))
            } else {
                Ok(BufrFile { src })
            }
        }
    }
}

impl Iterator for BufrFile {
    type Item = Message;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut error_code: libc::c_int = 0;
            let message: *mut codes_handle = codes_handle_new_from_file(
                std::ptr::null_mut(),
                self.src,
                ProductKind::PRODUCT_BUFR,
                &mut error_code,
            );
            if error_code != CODES_SUCCESS || message.is_null() {
                None
            } else {
                Message::new(message).ok()
            }
        }
    }
}
