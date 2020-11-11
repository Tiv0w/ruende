mod utils {
    pub fn u16_to_couple_of_u8(num: u16) -> (u8, u8) {
        ((num >> 8) as u8, (num & 0x00ff) as u8)
    }
    pub fn couple_of_u8_to_u16((a, b): (u8, u8)) -> u16 {
        ((a as u16) << 8) | (b as u16)
    }
}

pub fn decode(data: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    for idx in 0..data.len() {
        print!("| {} ", data[idx]);
    }

    let mut initial_dictionary_idx: Vec<Vec<u8>> = Vec::new();
    for ascii_symbol in 0..=0xff {
        initial_dictionary_idx.push(vec![ascii_symbol]);
    }
    let mut dictionary_idx: Vec<Vec<u8>> = initial_dictionary_idx.clone();
    let mut dictionary_len: u16 = 0x100;

    let mut reserved_byte_ready = false;
    let mut reserved_byte: u8 = 0;

    println!("\nON EST LA");

    let mut _old_code: u16;
    let initial_code = utils::couple_of_u8_to_u16((data[0], data[1]));
    let mut old_vec: Vec<u8> = vec![initial_code as u8];
    result.push(initial_code as u8);

    for &byte in &data[2..] {
        if dictionary_len == 0xffff {
            println!("- Dictionary flushed");
            dictionary_idx = initial_dictionary_idx.clone();
            dictionary_len = 0x100;
        }

        println!("tour: {} ", dictionary_len - 0x100);

        if !reserved_byte_ready {
            reserved_byte = byte;
            reserved_byte_ready = true;
        } else {
            let code = utils::couple_of_u8_to_u16((reserved_byte, byte));

            if code < dictionary_len {
                println!("Inférieur à dictionary_len: {}", code);
                let vec: &Vec<u8> = &dictionary_idx[code as usize];
                let vec_first_byte: u8 = vec[0];

                for &byte in vec {
                    result.push(byte);
                }

                old_vec.push(vec_first_byte);
                dictionary_idx.push(old_vec.clone());
                dictionary_len += 1;
            } else {
                println!("Supérieur à dictionary_len: {}", code);

                let old_first_byte: u8 = old_vec[0];

                old_vec.push(old_first_byte);

                for &byte in &old_vec {
                    result.push(byte);
                }

                dictionary_idx.push(old_vec.clone());
                dictionary_len += 1;
            }
            reserved_byte_ready = false;
        }
    }

    println!("RESULT: {}", String::from_utf8(result.clone()).unwrap());

    result
}
