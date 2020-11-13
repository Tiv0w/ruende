fn couple_of_u8_to_u16((a, b): (u8, u8)) -> u16 {
    ((a as u16) << 8) | (b as u16)
}

/// The main decoding method for LZW
pub fn decode(data: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    // Dictionary initialization
    let mut initial_dictionary: Vec<Vec<u8>> = Vec::new();
    let mut dictionary_len: u16 = 0x0100;
    for ascii_symbol in 0..dictionary_len {
        initial_dictionary.push(vec![ascii_symbol as u8]);
    }
    let mut dictionary: Vec<Vec<u8>> = initial_dictionary.clone();

    let mut old_code: u16 = couple_of_u8_to_u16((data[0], data[1]));
    result.push(old_code as u8);

    let mut data = data[2..].into_iter(); // Shadow data because we'll only use the iterator
    while let (Some(&first_byte), Some(&second_byte)) = (data.next(), data.next()) {
        // Reset the dictionary to the default one if full
        if dictionary_len == 0xffff {
            dictionary = initial_dictionary.clone();
            dictionary_len = 0x0100;
        }

        let current_code: u16 = couple_of_u8_to_u16((first_byte, second_byte));
        let mut old_vec: Vec<u8> = dictionary[old_code as usize].clone();

        if current_code < dictionary_len {
            let vec: &Vec<u8> = &dictionary[current_code as usize];
            // Write old + first byte of vec to result
            old_vec.push(vec[0]);
            for &byte in vec {
                result.push(byte);
            }
        } else {
            // Write old + first byte of old to result
            old_vec.push(old_vec[0]);
            for &byte in &old_vec {
                result.push(byte);
            }
        }
        // Add old + first_byte of [old or vec] to dictionary
        dictionary.push(old_vec);
        dictionary_len += 1;

        old_code = current_code;
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn couple_of_u8_to_u16() {
        assert_eq!(0x20, super::couple_of_u8_to_u16((0, 0x20)));
        assert_ne!(0x20, super::couple_of_u8_to_u16((0, 0x1f)));
        assert_eq!(0x100, super::couple_of_u8_to_u16((0x1, 0)));
        assert_ne!(0x100, super::couple_of_u8_to_u16((0x1, 0x1)));
    }
}
