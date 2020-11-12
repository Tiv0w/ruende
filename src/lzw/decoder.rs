fn couple_of_u8_to_u16((a, b): (u8, u8)) -> u16 {
    ((a as u16) << 8) | (b as u16)
}

pub fn decode(data: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    let mut initial_dictionary_idx: Vec<Vec<u8>> = Vec::new();
    let mut dictionary_len: u16 = 0x0100;
    for ascii_symbol in 0..dictionary_len {
        initial_dictionary_idx.push(vec![ascii_symbol as u8]);
    }
    let mut dictionary_idx: Vec<Vec<u8>> = initial_dictionary_idx.clone();

    let mut old_code: u16 = couple_of_u8_to_u16((data[0], data[1]));
    result.push(old_code as u8);

    let mut data = data[2..].into_iter();
    loop {
        if dictionary_len == 0xffff {
            dictionary_idx = initial_dictionary_idx.clone();
            dictionary_len = 0x0100;
        }

        match (data.next(), data.next()) {
            (Some(&first_byte), Some(&second_byte)) => {
                let current_code: u16 = couple_of_u8_to_u16((first_byte, second_byte));
                let mut old_vec: Vec<u8> = dictionary_idx[old_code as usize].clone();

                if current_code < dictionary_len {
                    let vec: &Vec<u8> = &dictionary_idx[current_code as usize];
                    old_vec.push(vec[0]);
                    for &byte in vec {
                        result.push(byte);
                    }
                    dictionary_idx.push(old_vec);
                    dictionary_len += 1;
                } else {
                    old_vec.push(old_vec[0]);
                    for &byte in &old_vec {
                        result.push(byte);
                    }
                    dictionary_idx.push(old_vec);
                    dictionary_len += 1;
                }

                old_code = current_code;
            }
            (_, _) => break,
        }
    }

    result
}
