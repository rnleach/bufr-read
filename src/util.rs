#[cfg(test)]
pub(crate) mod test {

    pub fn data_3p1d1() -> &'static [u8] {
        &[
            // Section 0
            0b01000010, // 'B'
            0b01010101, // 'U'
            0b01000110, // 'F'
            0b01010010, // 'R'
            0b00000000, // ---
            0b00000000, // Total length of message in octets as 24 bit number (52)
            0b00110100, // ---
            0b00000100, // BUFR edition number (4)
            // Section 1
            0b00000000, // ---
            0b00000000, // Section length in octets as 24 bit number (22)
            0b00010110, // ---^
            0b00000000, // BUFR master table (0)
            0b00000000, // Originating center (58)
            0b00111010, // ---^
            0b00000000, // Originating sub-center (0)
            0b00000000, // ---^
            0b00000000, // Table update number (0)
            0b00000000, // Section 2 flag (false)
            0b00000000, // Data category (0)
            0b00000000, // Data sub-category (0)
            0b00000000, // Local data sub-category (0)
            0b00001001, // Version of master tables (9)
            0b00000001, // Version of local tables (1)
            0b00000111, // Year of century (1 for 2001)
            0b11010001, // ---^
            0b00000100, // Month (4)
            0b00011101, // Day (29)
            0b00001100, // Hour (12)
            0b00000000, // Minute (0)
            0b00000000, // Rest of section.
            // Section 2
            // Section 3
            0b00000000, 0b00000000, 0b00001110, 0b00000000, 0b00000000, 0b00000001, 0b10000000,
            0b00000001, 0b00000001, 0b00000001, 0b00000010, 0b00001100, 0b00000100, 0b00000000,
            // Section 4
            0b00000000, 0b00000000, 0b00001000, 0b00000000, 0b10010000, 0b11110101, 0b11011100,
            0b01000000, // Section 5
            0b00110111, // '7'
            0b00110111, // '7'
            0b00110111, // '7'
            0b00110111, // '7'
        ]
    }
}
