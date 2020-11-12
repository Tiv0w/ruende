use std::collections::HashMap;

mod utils {
    pub fn u16_to_couple_of_u8(num: u16) -> (u8, u8) {
        ((num >> 8) as u8, (num & 0x00ff) as u8)
    }
    pub fn couple_of_u8_to_u16((a, b): (u8, u8)) -> u16 {
        ((a as u16) << 8) | (b as u16)
    }
}

pub fn encode(data: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    let mut initial_dictionary: HashMap<Vec<u8>, u16> = HashMap::default();
    for ascii_symbol in 0..=0xff {
        initial_dictionary.insert(vec![ascii_symbol], ascii_symbol as u16);
    }
    let mut dictionary: HashMap<Vec<u8>, u16> = initial_dictionary.clone();
    let mut dictionary_len: u16 = 0x100;

    let mut working_bytes: Vec<u8> = Vec::new();

    for &value in data {
        if dictionary_len == 0xffff {
            dictionary = initial_dictionary.clone();
            dictionary_len = 0x100;
        }

        let mut working_bytes_plus_current_byte = working_bytes.clone();
        working_bytes_plus_current_byte.push(value);

        let code: Result<u16, _> = get_code_from_vec(&working_bytes_plus_current_byte, &dictionary);

        match code {
            Ok(_) => {
                working_bytes = working_bytes_plus_current_byte;
            }
            Err(_) => {
                let current_code: u16 = get_code_from_vec(&working_bytes, &dictionary).unwrap();

                let (a, b) = utils::u16_to_couple_of_u8(current_code);
                result.push(a);
                result.push(b);

                dictionary.insert(working_bytes_plus_current_byte, dictionary_len);
                dictionary_len += 1;
                working_bytes = Vec::from([value]);
            }
        }
    }

    let current_code = get_code_from_vec(&working_bytes, &dictionary).unwrap();
    let (a, b) = utils::u16_to_couple_of_u8(current_code);
    result.push(a);
    result.push(b);

    result
}

fn get_code_from_vec(vec: &Vec<u8>, dictionary: &HashMap<Vec<u8>, u16>) -> Result<u16, ()> {
    if !dictionary.contains_key(vec) {
        return Err(());
    }

    Ok(*dictionary.get(vec).unwrap())
}
