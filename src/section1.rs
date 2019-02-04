use byteorder::{BigEndian, ReadBytesExt};
use std::{error::Error, io::Read};

/// Section 1 of a bufr
#[derive(Debug, Default)]
pub struct Section1 {
    pub section_length: u32,
    pub bufr_master_table: u8,
    pub orig_center: u16,
    pub orig_sub_center: u16,
    pub update: u8,
    pub section_2: bool,
    pub data_cat: u8,
    pub data_sub_cat: u8,
    pub local_data_sub_cat: u8,
    pub version_num_master_table: u8,
    pub version_num_local_table: u8,
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub extra: Vec<u8>,
}

impl Section1 {
    /// Read section 1.
    ///
    // This function assumes the byte stream is positioned to start reading section 1 next.
    pub fn read<R: Read>(mut src: R) -> Result<Section1, Box<dyn Error>> {
        let mut section1 = Section1::default();

        section1.section_length = src.read_u24::<BigEndian>()?;
        section1.bufr_master_table = src.read_u8()?;
        section1.orig_center = src.read_u16::<BigEndian>()?;
        section1.orig_sub_center = src.read_u16::<BigEndian>()?;
        section1.update = src.read_u8()?;
        section1.section_2 = src.read_u8()? & 0b1000_0000 != 0;
        section1.data_cat = src.read_u8()?;
        section1.data_sub_cat = src.read_u8()?;
        section1.local_data_sub_cat = src.read_u8()?;
        section1.version_num_master_table = src.read_u8()?;
        section1.version_num_local_table = src.read_u8()?;
        section1.year = src.read_u16::<BigEndian>()?;
        section1.month = src.read_u8()?;
        section1.day = src.read_u8()?;
        section1.hour = src.read_u8()?;
        section1.minute = src.read_u8()?;

        let remaining = (section1.section_length - 21) as usize;

        section1.extra = vec![0; remaining];
        src.read_exact(&mut section1.extra)?;

        Ok(section1)
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

        let sec1 = Section1::read(&mut data).unwrap();
        assert_eq!(sec1.section_length, 22);
        assert_eq!(sec1.bufr_master_table, 0);
        assert_eq!(sec1.orig_sub_center, 0);
        assert_eq!(sec1.orig_center, 58);
        assert_eq!(sec1.update, 0);
        assert_eq!(sec1.section_2, false);
        assert_eq!(sec1.data_cat, 0);
        assert_eq!(sec1.data_sub_cat, 0);
        assert_eq!(sec1.version_num_master_table, 9);
        assert_eq!(sec1.version_num_local_table, 1);
        assert_eq!(sec1.year, 2001);
        assert_eq!(sec1.month, 4);
        assert_eq!(sec1.day, 29);
        assert_eq!(sec1.hour, 12);
        assert_eq!(sec1.minute, 0);
        assert_eq!(sec1.extra, vec![0]);
    }
}
