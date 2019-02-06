use super::Message;
use crate::error::BufrErr;
use eccodes_sys::{
    codes_get_error_message, codes_keys_iterator, codes_keys_iterator_delete,
    codes_keys_iterator_get_name, codes_keys_iterator_new, codes_keys_iterator_next,
    CODES_KEYS_ITERATOR_ALL_KEYS, CODES_SUCCESS,
};
use std::{borrow::Cow, ffi::CStr, marker::PhantomData, ptr};

pub struct KeysIterator<'a> {
    iter: *mut codes_keys_iterator,
    _phantom: PhantomData<&'a Message>,
}

impl<'a> Drop for KeysIterator<'a> {
    fn drop(&mut self) {
        unsafe {
            let err_code = codes_keys_iterator_delete(self.iter);
            if err_code != CODES_SUCCESS {
                let msg: &CStr = &CStr::from_ptr(codes_get_error_message(err_code));
                eprintln!("{}", msg.to_string_lossy());

                panic!("error from eccodes while deleting iterator key");
            }
        }
    }
}

impl<'a> KeysIterator<'a> {
    pub(crate) fn new(msg: &'a Message) -> Result<Self, BufrErr> {
        unsafe {
            let ptr: *mut codes_keys_iterator =
                codes_keys_iterator_new(msg.handle, CODES_KEYS_ITERATOR_ALL_KEYS, ptr::null_mut());

            if ptr.is_null() {
                Err(BufrErr::NullPtr)
            } else {
                Ok(KeysIterator {
                    iter: ptr,
                    _phantom: PhantomData,
                })
            }
        }
    }
}

impl<'a> Iterator for KeysIterator<'a> {
    type Item = Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if 0 == codes_keys_iterator_next(self.iter) {
                None
            } else {
                let name: *const libc::c_char = codes_keys_iterator_get_name(self.iter);
                let name = CStr::from_ptr(name);
                Some(name.to_string_lossy())
            }
        }
    }
}
