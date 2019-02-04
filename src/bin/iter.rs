use bufr::{skip_to_message_start, Section0};
use eccodes_sys::*;
use std::{
    env,
    ffi::CStr,
    fs::File,
    io::{BufReader, Cursor, Read},
    ptr::null_mut,
};

macro_rules! codes_check {
    ($err_code:expr, $tgt_code:expr) => {
        if $err_code != $tgt_code as libc::c_int {
            return Err(format!("ECCODES ERROR: {}", $err_code))?;
        }
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for arg in env::args().skip(1) {
        let mut f = BufReader::new(File::open(arg)?);

        let mut buf: Vec<u8> = Vec::with_capacity(30000);
        f.read_to_end(&mut buf)?;

        let mut cursor = Cursor::new(buf);
        skip_to_message_start(&mut cursor)?;
        let pos = cursor.position() as usize;
        let length = Section0::read(&mut cursor)?.message_length as usize;
        let mut buf = cursor.into_inner();
        let slice = &mut buf[pos..(pos + length)];

        unsafe {
            let hndl = codes_handle_new_from_message(
                null_mut(),
                slice.as_mut_ptr(),
                slice.len() as libc::size_t,
            );

            if hndl.is_null() {
                println!("Another null pointer");
                return Err(format!("ECCODES ERROR: null pointer"))?;
            }

            codes_check!(codes_set_long(hndl, "unpack\0".as_ptr() as *const i8, 1), 0);

            let kiter = codes_bufr_keys_iterator_new(hndl, 0);
            if kiter.is_null() {
                println!("Error getting iterator");
                return Err(format!("ECCODES ERROR: iterator null pointer"))?;
            }

            while codes_bufr_keys_iterator_next(kiter) != 0 {
                /* get key name */
                let name: &CStr = &CStr::from_ptr(codes_bufr_keys_iterator_get_name(kiter));
                print!("  {}=", name.to_string_lossy());

                let mut klen: libc::size_t = 0;
                codes_check!(codes_get_size(hndl, name.as_ptr(), &mut klen), 0);

                if klen == 1 {
                    let mut value: libc::c_double = 0.0;
                    codes_check!(codes_get_double(hndl, name.as_ptr(), &mut value), 0);

                    let value = if value == CODES_MISSING_DOUBLE {
                        None
                    } else {
                        Some(value)
                    };

                    println!("{:?}", value);
                } else {
                    println!("array of length {}", klen);
                }
            }

            codes_check!(codes_bufr_keys_iterator_delete(kiter), 0);
            codes_check!(codes_handle_delete(hndl), 0);
        };
    }
    Ok(())
}
