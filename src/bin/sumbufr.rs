use bufr::*;
use std::{env, fs::File, io::BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for arg in env::args().skip(1) {
        println!("File: {}", arg);
        let mut reader = BufReader::new(File::open(&arg)?);

        skip_to_message_start(&mut reader)?;

        let sec0 = Section0::read(&mut reader)?;
        let sec1 = Section1::read(&mut reader)?;
        let sec2: Option<Section2> = if sec1.section_2 {
            Some(Section2::read(&mut reader)?)
        } else {
            None
        };
        let sec3 = Section3::read(&mut reader)?;

        println!("{:#?}", sec0);
        println!("{:#?}", sec1);
        if let Some(sec2) = sec2 {
            println!("Section 2 length: {}", sec2.section_length);
        } else {
            println!("No section 2 data.");
        }
        println!("{}", sec3);

        assert_eq!(
            sec1.data_cat, 2,
            "Only looking at land upper air at this time."
        );

        println!("---------------------------------------------------------------------------\n");
    }

    Ok(())
}
