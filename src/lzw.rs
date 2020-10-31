use crate::utils;
use std::cmp::Ordering;

pub fn encode(data: &Vec<u8>) -> (Vec<u8>, Vec<u32>) {
    let mut result32: Vec<u32> = Vec::new();
    let mut result: Vec<u8> = Vec::new();
    let mut dictionary: Vec<Vec<u8>> = Vec::new();
    for symbol in 0..=255 {
        let mut a: Vec<u8> = Vec::new();
        a.push(symbol);
        dictionary.push(a);
    }

    println!("{:?}", dictionary);

    let mut is_code_ready = false;
    let mut code_ready: u32 = 0;
    let mut working_bytes: Vec<u8> = Vec::new();

    for &value in data {
        print!("{} ||", value);

        let mut working_bytes_plus_current_byte = working_bytes.clone();
        working_bytes_plus_current_byte.push(value);

        let code: Result<u32, _> =
            get_vec_code_from_dictionary(&working_bytes_plus_current_byte, &dictionary);

        println!(
            "Working bytes plus: {:?}, Code result: {:?}",
            working_bytes_plus_current_byte, code
        );

        match code {
            Ok(_) => {
                working_bytes = working_bytes_plus_current_byte;
            }
            Err(_) => {
                let current_code: u32 =
                    get_vec_code_from_dictionary(&working_bytes, &dictionary).unwrap();
                if is_code_ready {
                    println!(" Code: {}", current_code);
                    let (a, b, c) = utils::couple_to_triple((code_ready, current_code));
                    result.append(&mut Vec::from([a, b, c]));
                    result32.append(&mut Vec::from([code_ready, current_code]));
                    is_code_ready = false;
                } else {
                    code_ready = current_code;
                    println!(" Code: {}", code_ready);
                    is_code_ready = true;
                }

                dictionary.push(working_bytes_plus_current_byte);
                working_bytes = Vec::from([value]);
            }
        }
    }

    let current_code = get_vec_code_from_dictionary(&working_bytes, &dictionary).unwrap();
    if is_code_ready {
        let (a, b, c) = utils::couple_to_triple((code_ready, current_code));
        result.append(&mut Vec::from([a, b, c]));
        result32.append(&mut Vec::from([code_ready, current_code]));
    } else {
        let (a, b, c) = utils::couple_to_triple((current_code, 0));
        result.append(&mut Vec::from([a, b, c]));
        result32.append(&mut Vec::from([current_code]));
    }

    (result, result32)
}

fn get_vec_code_from_dictionary(vec: &Vec<u8>, dictionary: &Vec<Vec<u8>>) -> Result<u32, ()> {
    for (i, code) in dictionary.iter().enumerate() {
        if vec.cmp(code) == Ordering::Equal {
            return Ok(i as u32);
        }
    }
    Err(())
}
