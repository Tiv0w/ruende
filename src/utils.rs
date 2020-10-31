pub use couples_and_triples::couple_to_triple;
pub use couples_and_triples::triple_to_couple;

pub mod nibble {
    /// Returns a byte with the most significant nibble set to 0x0
    pub fn read_least(o: u8) -> u8 {
        o & 0x0f
    }

    #[allow(dead_code)]
    /// Returns a byte with the least significant nibble set to 0x0
    pub fn read_most(o: u8) -> u8 {
        o & 0xf0
    }

    #[allow(dead_code)]
    /// Returns a byte with ln as least significant nibble
    pub fn write_least(o: u8, ln: u8) -> u8 {
        (o & 0xf0) | ln
    }

    /// Returns a byte with mn as most significant nibble
    pub fn write_most(o: u8, mn: u8) -> u8 {
        (o & 0x0f) | mn
    }
}

mod couples_and_triples {
    use super::nibble;

    /// Returns a triple of
    pub fn couple_to_triple((a, b): (u32, u32)) -> (u8, u8, u8) {
        let third_byte: u8 = (b & 0xff) as u8;

        let second_byte_least_nibble = nibble::read_least((b >> 8) as u8);
        let second_byte: u8 = nibble::write_most(second_byte_least_nibble, (b >> 8) as u8);

        let first_byte: u8 = (a >> 4) as u8;

        (first_byte, second_byte, third_byte)
    }

    #[allow(dead_code)]
    pub fn triple_to_couple((a, b, c): (u8, u8, u8)) -> (u32, u32) {
        let first_word: u32 = ((a << 4) as u32) | ((nibble::read_most(b) >> 4) as u32);

        let second_word: u32 = (nibble::read_least(b) as u32) << 8 | c as u32;

        (first_word, second_word)
    }
}
