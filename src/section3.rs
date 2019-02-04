use byteorder::{BigEndian, ReadBytesExt};
use std::{error::Error, fmt::Display, io::Read};

/// Section 3 of a BUFR message
#[derive(Debug, Default)]
pub struct Section3 {
    pub section_length: u32,
    pub num_subsets: u16,
    pub observed: bool,
    pub compressed: bool,
    pub descriptors: Vec<Descriptor>,
}

impl Section3 {
    /// Read section 3.
    ///
    // This function assumes the byte stream is positioned to start reading section 3 next.
    pub fn read<R: Read>(mut src: R) -> Result<Section3, Box<dyn Error>> {
        let mut sec3 = Section3::default();

        sec3.section_length = src.read_u24::<BigEndian>()?;
        let _ = src.read_u8()?;
        sec3.num_subsets = src.read_u16::<BigEndian>()?;

        let bitfield = src.read_u8()?;
        sec3.observed = bitfield & 0b1000_0000 != 0;
        sec3.compressed = bitfield & 0b0100_0000 != 0;

        let num_descriptors: usize = ((sec3.section_length - 7 + 1) / 2) as usize;
        sec3.descriptors = Vec::with_capacity(num_descriptors);

        for _ in 0..num_descriptors {
            sec3.descriptors
                .push(Descriptor::new(src.read_u16::<BigEndian>()?));
        }

        Ok(sec3)
    }
}

impl Display for Section3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(
            f,
            "\nSection3: Length = {} Number of subsets = {} Observed = {} Compressed = {}",
            self.section_length, self.num_subsets, self.observed, self.compressed
        )?;

        for desc in &self.descriptors {
            writeln!(f, "     {}", desc)?;
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct Descriptor(u16);

impl Descriptor {
    pub fn new(bytes: u16) -> Self {
        Descriptor(bytes)
    }

    pub fn f(&self) -> u8 {
        ((self.0 & 0b1100_0000_0000_0000) >> 14) as u8
    }

    pub fn x(&self) -> u8 {
        ((self.0 & 0b0011_1111_0000_0000) >> 8) as u8
    }

    pub fn y(&self) -> u8 {
        (self.0 & 0b0000_0000_1111_1111) as u8
    }
}

impl Display for Descriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "F:{:1} X:{:02} Y:{:03}", self.f(), self.x(), self.y())
    }
}

// #[cfg(test)]
// mod test {

//     use crate::util::test::*;
//     use crate::*;
//     use std::io::Cursor;

//     #[test]
//     fn test_section1() {
//         let mut data = Cursor::new(data_3p1d1());
//         skip_to_message_start(&mut data).unwrap();
//         let _ = Section0::read(&mut data).unwrap();

//         let sec1 = Section1::read(&mut data).unwrap();
//         assert_eq!(sec1.section_length, 22);
//         assert_eq!(sec1.bufr_master_table, 0);
//         assert_eq!(sec1.orig_sub_center, 0);
//         assert_eq!(sec1.orig_center, 58);
//         assert_eq!(sec1.update, 0);
//         assert_eq!(sec1.section_2, false);
//         assert_eq!(sec1.data_cat, 0);
//         assert_eq!(sec1.data_sub_cat, 0);
//         assert_eq!(sec1.version_num_master_table, 9);
//         assert_eq!(sec1.version_num_local_table, 1);
//         assert_eq!(sec1.year, 2001);
//         assert_eq!(sec1.month, 4);
//         assert_eq!(sec1.day, 29);
//         assert_eq!(sec1.hour, 12);
//         assert_eq!(sec1.minute, 0);
//         assert_eq!(sec1.extra, vec![0]);
//     }
// }
