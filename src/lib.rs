//
// API
//
pub use crate::section0::Section0;
pub use crate::section1::Section1;
pub use crate::section2::Section2;
pub use crate::section3::Section3;

/// Find the start of a message.
pub fn skip_to_message_start<R: Read + Seek>(mut src: R) -> Result<(), Box<dyn Error>> {
    let mut buf = [0u8; 1];
    let mut b = false;
    let mut u = false;
    let mut f = false;

    while let Ok(n) = src.read(&mut buf) {
        if n != 1 {
            return Err(Box::<dyn Error>::from("No BUFR message found."));
        }

        if buf == [b'B'] {
            b = true;
        } else if b && buf == [b'U'] {
            u = true;
        } else if u && buf == [b'F'] {
            f = true;
        } else if f && buf == [b'R'] {
            src.seek(SeekFrom::Current(-4))?;
            break;
        }
    }

    Ok(())
}

//
// Internal only
//
mod section0;
mod section1;
mod section2;
mod section3;
mod util;

use std::{
    error::Error,
    io::{Read, Seek, SeekFrom},
};
