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
    let mut number_of_dictionary_flushes = 0;

    let mut initial_dictionary: HashMap<Vec<u8>, u16> = HashMap::default();
    for ascii_symbol in 0..=0xff {
        initial_dictionary.insert(vec![ascii_symbol], ascii_symbol as u16);
    }
    let mut dictionary: HashMap<Vec<u8>, u16> = initial_dictionary.clone();
    let mut dictionary_len: u16 = 0x100;

    let mut working_bytes: Vec<u8> = Vec::new();

    for &value in data {
        if dictionary_len == 0xffff {
            println!("- Dictionary flushed");
            number_of_dictionary_flushes += 1;
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

    println!(
        "Number of dictionary flushes: {}",
        number_of_dictionary_flushes
    );

    result
}

fn get_code_from_vec(vec: &Vec<u8>, dictionary: &HashMap<Vec<u8>, u16>) -> Result<u16, ()> {
    if !dictionary.contains_key(vec) {
        return Err(());
    }

    Ok(*dictionary.get(vec).unwrap())
}

fn get_vec_from_code(code: &u16, dictionary: &HashMap<u16, Vec<u8>>) -> Result<Vec<u8>, ()> {
    if !dictionary.contains_key(code) {
        return Err(());
    }

    Ok((*dictionary.get(code).unwrap()).clone())
}

pub fn decode(data: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    let mut initial_dictionary_vec: HashMap<Vec<u8>, u16> = HashMap::default();
    let mut initial_dictionary_idx: HashMap<u16, Vec<u8>> = HashMap::default();
    for ascii_symbol in 0..=0xff {
        initial_dictionary_vec.insert(vec![ascii_symbol], ascii_symbol as u16);
        initial_dictionary_idx.insert(ascii_symbol as u16, vec![ascii_symbol]);
    }
    let mut dictionary_vec: HashMap<Vec<u8>, u16> = initial_dictionary_vec.clone();
    let mut dictionary_idx: HashMap<u16, Vec<u8>> = initial_dictionary_idx.clone();
    let mut dictionary_len: u16 = 0x100;

    let mut working_bytes: Vec<u8> = Vec::new();

    let mut reserved_byte_ready = false;
    let mut reserved_byte: u8 = 0;

    for &byte in data {
        if dictionary_len == 0xffff {
            println!("- Dictionary flushed");
            dictionary_vec = initial_dictionary_vec.clone();
            dictionary_idx = initial_dictionary_idx.clone();
        }

        if !reserved_byte_ready {
            reserved_byte = byte;
            reserved_byte_ready = true;
        } else {
            let code = utils::couple_of_u8_to_u16((reserved_byte, byte));

            if code < 0x100 {
                result.push(code as u8);

                let mut working_bytes_plus_current_byte = working_bytes.clone();
                working_bytes_plus_current_byte.push(code as u8);

                let current_code_test: Result<u16, _> =
                    get_code_from_vec(&working_bytes_plus_current_byte, &dictionary_vec);
                match current_code_test {
                    Ok(_) => {
                        working_bytes = working_bytes_plus_current_byte;
                    }
                    Err(_) => {
                        dictionary_vec
                            .insert(working_bytes_plus_current_byte.clone(), dictionary_len);
                        dictionary_idx.insert(dictionary_len, working_bytes_plus_current_byte);
                        dictionary_len += 1;

                        working_bytes = vec![code as u8];
                    }
                }
            } else {
                let vec: Vec<u8> = get_vec_from_code(&code, &dictionary_idx).unwrap();
                for byte in vec {
                    result.push(byte);

                    let mut working_bytes_plus_current_byte = working_bytes.clone();
                    working_bytes_plus_current_byte.push(byte);

                    let current_code_test: Result<u16, _> =
                        get_code_from_vec(&working_bytes_plus_current_byte, &dictionary_vec);
                    match current_code_test {
                        Ok(_) => {
                            working_bytes = working_bytes_plus_current_byte;
                        }
                        Err(_) => {
                            dictionary_vec
                                .insert(working_bytes_plus_current_byte.clone(), dictionary_len);
                            dictionary_idx.insert(dictionary_len, working_bytes_plus_current_byte);
                            dictionary_len += 1;

                            working_bytes = vec![byte];
                        }
                    }
                }
            }

            reserved_byte_ready = false;
        }
    }

    result
}
