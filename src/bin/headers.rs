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
                continue;
            }

            let mut long_val: libc::c_long = 0;

            let key: &CStr = &CStr::from_bytes_with_nul(b"dataCategory\0")?;
            codes_check!(codes_get_long(hndl, key.as_ptr(), &mut long_val), 0);
            println!("  {:#?}: {}", key, long_val);

            let key: &CStr = &CStr::from_bytes_with_nul(b"dataSubCategory\0")?;
            codes_check!(codes_get_long(hndl, key.as_ptr(), &mut long_val), 0);
            println!("  {:#?}: {}", key, long_val);

            let key: &CStr = &CStr::from_bytes_with_nul(b"typicalDate\0")?;
            codes_check!(codes_get_long(hndl, key.as_ptr(), &mut long_val), 0);
            println!("  {:#?}: {}", key, long_val);

            let key: &CStr = &CStr::from_bytes_with_nul(b"bufrHeaderCentre\0")?;
            codes_check!(codes_get_long(hndl, key.as_ptr(), &mut long_val), 0);
            println!("  {:#?}: {}", key, long_val);

            let key: &CStr = &CStr::from_bytes_with_nul(b"bufrHeaderSubCentre\0")?;
            codes_check!(codes_get_long(hndl, key.as_ptr(), &mut long_val), 0);
            println!("  {:#?}: {}", key, long_val);

            let key: &CStr = &CStr::from_bytes_with_nul(b"masterTablesVersionNumber\0")?;
            codes_check!(codes_get_long(hndl, key.as_ptr(), &mut long_val), 0);
            println!("  {:#?}: {}", key, long_val);

            let key: &CStr = &CStr::from_bytes_with_nul(b"localTablesVersionNumber\0")?;
            codes_check!(codes_get_long(hndl, key.as_ptr(), &mut long_val), 0);
            println!("  {:#?}: {}", key, long_val);

            let key: &CStr = &CStr::from_bytes_with_nul(b"numberOfSubsets\0")?;
            codes_check!(codes_get_long(hndl, key.as_ptr(), &mut long_val), 0);
            println!("  {:#?}: {}", key, long_val);

            codes_handle_delete(hndl);
        }
    }

    Ok(())
}
