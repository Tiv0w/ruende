use std::cmp::Ordering;

mod utils {
    pub fn u16_to_couple_of_u8(num: u16) -> (u8, u8) {
        ((num >> 8) as u8, (num & 0x00ff) as u8)
    }
}

pub fn encode(data: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut number_of_dictionary_flushes = 0;

    let mut initial_dictionary: Vec<Vec<u8>> = Vec::new();
    for ascii_symbol in 0..=255 {
        let ascii_symbol: Vec<u8> = vec![ascii_symbol];
        initial_dictionary.push(ascii_symbol);
    }
    let mut dictionary: Vec<Vec<u8>> = initial_dictionary.clone();

    let mut working_bytes: Vec<u8> = Vec::new();

    for &value in data {
        if dictionary.len() as u16 == (2u32.pow(16) - 1) as u16 {
            println!("- Dictionary flush");
            number_of_dictionary_flushes += 1;
            dictionary = initial_dictionary.clone();
        }

        let mut working_bytes_plus_current_byte = working_bytes.clone();
        working_bytes_plus_current_byte.push(value);

        let code: Result<u16, _> =
            get_vec_code_from_dictionary(&working_bytes_plus_current_byte, &dictionary);

        match code {
            Ok(_) => {
                working_bytes = working_bytes_plus_current_byte;
            }
            Err(_) => {
                let current_code: u16 =
                    get_vec_code_from_dictionary(&working_bytes, &dictionary).unwrap();

                let (a, b) = utils::u16_to_couple_of_u8(current_code);
                result.push(a);
                result.push(b);

                dictionary.push(working_bytes_plus_current_byte);
                working_bytes = Vec::from([value]);
            }
        }
    }

    let current_code = get_vec_code_from_dictionary(&working_bytes, &dictionary).unwrap();
    let (a, b) = utils::u16_to_couple_of_u8(current_code);
    result.push(a);
    result.push(b);

    println!(
        "Number of dictionary flushes: {}",
        number_of_dictionary_flushes
    );

    result
}

fn get_vec_code_from_dictionary(vec: &Vec<u8>, dictionary: &Vec<Vec<u8>>) -> Result<u16, ()> {
    for (i, code) in dictionary.iter().enumerate() {
        if vec.cmp(code) == Ordering::Equal {
            return Ok(i as u16);
        }
    }
    Err(())
}
