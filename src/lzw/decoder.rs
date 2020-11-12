fn couple_of_u8_to_u16((a, b): (u8, u8)) -> u16 {
    ((a as u16) << 8) | (b as u16)
}

pub fn decode(data: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    let mut initial_dictionary: Vec<Vec<u8>> = Vec::new();
    let mut dictionary_len: u16 = 0x0100;
    for ascii_symbol in 0..dictionary_len {
        initial_dictionary.push(vec![ascii_symbol as u8]);
    }
    let mut dictionary: Vec<Vec<u8>> = initial_dictionary.clone();

    let mut old_code: u16 = couple_of_u8_to_u16((data[0], data[1]));
    result.push(old_code as u8);

    let mut data = data[2..].into_iter();

    while let (Some(&first_byte), Some(&second_byte)) = (data.next(), data.next()) {
        if dictionary_len == 0xffff {
            dictionary = initial_dictionary.clone();
            dictionary_len = 0x0100;
        }

        let current_code: u16 = couple_of_u8_to_u16((first_byte, second_byte));
        let mut old_vec: Vec<u8> = dictionary[old_code as usize].clone();

        if current_code < dictionary_len {
            let vec: &Vec<u8> = &dictionary[current_code as usize];
            old_vec.push(vec[0]);
            for &byte in vec {
                result.push(byte);
            }
            dictionary.push(old_vec);
            dictionary_len += 1;
        } else {
            old_vec.push(old_vec[0]);
            for &byte in &old_vec {
                result.push(byte);
            }
            dictionary.push(old_vec);
            dictionary_len += 1;
        }

        old_code = current_code;
    }

    result
}
