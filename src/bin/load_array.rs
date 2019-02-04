use bufr::{skip_to_message_start, Section0};
use eccodes_sys;
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
            let hndl = eccodes_sys::codes_handle_new_from_message(
                null_mut(),
                slice.as_mut_ptr(),
                slice.len() as libc::size_t,
            );

            if hndl.is_null() {
                println!("Another null pointer");
                return Err(format!("ECCODES ERROR: null pointer"))?;
            }

            codes_check!(eccodes_sys::codes_set_long(hndl, "unpack\0".as_ptr() as *const i8, 1), 0);

            let name: &CStr = &CStr::from_bytes_with_nul(b"pressure\0")?;

            let mut klen: libc::size_t = 0;
            codes_check!(eccodes_sys::codes_get_size(hndl, name.as_ptr(), &mut klen), 0);

            println!("array length is {}", klen);

            let mut vals: Vec<f64> = vec![0.0; klen as usize];
            codes_check!(eccodes_sys::codes_get_double_array(hndl, name.as_ptr(), vals.as_mut_ptr(), &mut klen), 0);

            println!("{:#?}", vals);

            codes_check!(eccodes_sys::codes_handle_delete(hndl), 0);
        };
    }
    Ok(())
}
