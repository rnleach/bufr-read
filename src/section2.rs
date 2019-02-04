use byteorder::{BigEndian, ReadBytesExt};
use std::{error::Error, io::Read};

/// Section 2 of a bufr
#[derive(Debug, Default)]
pub struct Section2 {
    pub section_length: u32,
    pub data: Vec<u8>,
}

impl Section2 {
    /// Read section 2.
    ///
    /// This function assumes that there is a section 2 in the message and that the byte stream is
    /// positioned to read it next.
    pub fn read<R: Read>(mut src: R) -> Result<Section2, Box<dyn Error>> {
        let mut section2 = Section2::default();

        section2.section_length = src.read_u24::<BigEndian>()?;
        assert_eq!(src.read_u8()?, 0);

        let remaining = (section2.section_length - 4) as usize;

        section2.data = vec![0; remaining];
        src.read_exact(&mut section2.data)?;

        Ok(section2)
    }
}

#[cfg(test)]
mod test {

    use crate::util::test::*;
    use crate::*;
    use std::io::Cursor;

    #[test]
    fn test_section1() {
        let mut data = Cursor::new(data_3p1d1());
        skip_to_message_start(&mut data).unwrap();
        let _ = Section0::read(&mut data).unwrap();
        let _ = Section1::read(&mut data).unwrap();

        // No section 2 in data_3p1d1
    }
}
