use byteorder::{BigEndian, ReadBytesExt};
use std::{error::Error, io::Read};

/// Section 0 of a bufr file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Section0 {
    pub message_length: u32,
    pub bufr_edition: u8,
}

impl Section0 {
    /// Read section 0.
    pub fn read<R: Read>(mut src: R) -> Result<Section0, Box<dyn Error>> {
        let mut buf = [0u8; 4];
        src.read_exact(&mut buf)?;

        // Check for BUFR characters at beginning.
        if buf[0..=3] != ['B' as u8, 'U' as u8, 'F' as u8, 'R' as u8] {
            let string = String::from_utf8(buf[0..=3].to_vec()).unwrap();
            let msg = format!("File does not start with 'BUFR': {:?}", string);
            return Err(Box::<Error>::from(msg));
        }

        // Read a 24 bit unsigned big endian integer
        let message_length = src.read_u24::<BigEndian>()?;
        let bufr_edition = src.read_u8()?;
        assert_eq!(bufr_edition, 4, "Only reading bufr edition 4 implemented.");

        Ok(Section0 {
            message_length,
            bufr_edition,
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::util::test::*;
    use crate::*;
    use std::io::Cursor;

    #[test]
    fn test_read_section_0() {
        let mut data = Cursor::new(data_3p1d1());
        skip_to_message_start(&mut data).unwrap();
        let sec0 = Section0::read(&mut data).unwrap();

        println!("{:#?}", sec0);
        assert_eq!(
            sec0,
            Section0 {
                message_length: 52,
                bufr_edition: 4
            }
        );
    }
}
