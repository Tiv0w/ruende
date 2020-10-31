pub mod utils {
    pub use super::private_utils::escape_delimiter;
}

static DELIMITER: u8 = b'#';

mod private_utils {
    /// Escapes the given delimiter in data.
    pub fn escape_delimiter(data: &Vec<u8>, delimiter: u8) -> Vec<u8> {
        let mut processed_data: Vec<u8> = Vec::new();

        for byte in data.iter() {
            if *byte == delimiter {
                processed_data.push(delimiter);
                processed_data.push(delimiter);
            } else {
                processed_data.push(*byte);
            }
        }

        processed_data
    }

    /// Turns a byte into a string of decimal digits representation
    pub fn digits_from_byte(byte: u8) -> Vec<u8> {
        let third_digit = byte % 10;
        let second_digit = (byte % 100) / 10;
        let first_digit = byte / 100;
        let array = [first_digit, second_digit, third_digit];

        let mut non_zero = false;
        let processed: Vec<u8> = array
            .iter()
            .filter(|&&x| {
                non_zero = non_zero || x != 0;
                non_zero || x != 0
            })
            .map(|&x| x + b'0')
            .collect();

        processed
    }
}

/// Encodes the given data
pub fn encode_with_delimiter(data: &Vec<u8>, educational_mode: bool) -> Vec<u8> {
    let preprocessed_data = utils::escape_delimiter(data, DELIMITER);
    encode_with_delimiter_logic(&preprocessed_data, educational_mode)
}

/// Encodes without any other form of treatment
fn encode_with_delimiter_logic(data: &Vec<u8>, educational_mode: bool) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut last_byte = data[0];
    let mut last_byte_counter = 1;

    let prepared_bytes = data[1..].iter().copied().chain(std::iter::once(0 as u8));

    for byte in prepared_bytes {
        if byte != last_byte {
            if last_byte_counter == 1 {
                result.push(last_byte);
            } else if last_byte_counter > 1 {
                result.push(DELIMITER);
                result.push(last_byte);
                if educational_mode {
                    result.append(&mut private_utils::digits_from_byte(last_byte_counter));
                } else {
                    result.push(last_byte_counter);
                }
                result.push(DELIMITER);
            }
            last_byte = byte;
            last_byte_counter = 1;
        } else if byte == DELIMITER {
            result.push(DELIMITER);
        } else {
            last_byte_counter += 1;
        }
    }

    result
}

/// Decodes the given data
pub fn decode_with_delimiter(data: &Vec<u8>) -> Vec<u8> {
    decode_with_delimiter_logic(data)
    // utils::escape_delimiter(&deprocessed_data, DELIMITER)
}

/// Decodes without any other form of treatment
fn decode_with_delimiter_logic(data: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut last_byte = data[0];
    let mut ignore_last = false;
    let mut in_sequence = false;

    // let prepared_bytes = data[1..].iter().map(|x| *x).chain(std::iter::once(0 as u8));

    for &byte in data[1..].iter() {
        if ignore_last {
            last_byte = byte;
            ignore_last = false;
            in_sequence = false;
            continue;
        }

        if byte != DELIMITER {
            if last_byte == DELIMITER {
                in_sequence = true;
                last_byte = byte;
            } else if last_byte != DELIMITER {
                if in_sequence {
                    result.append(&mut [last_byte].repeat(byte as usize));
                } else {
                    result.push(last_byte);
                    last_byte = byte;
                }
            }
        } else if in_sequence {
            in_sequence = false;
            last_byte = byte;
        } else if last_byte != DELIMITER {
            result.push(last_byte);
            last_byte = byte;
        } else {
            result.push(DELIMITER);
            ignore_last = true;
        }
    }

    result
}

/// Encodes the given data
pub fn encode(data: &Vec<u8>, educational_mode: bool) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut last_byte = data[0];
    let mut last_byte_counter = 1;

    let prepared_bytes = data[1..].iter().copied().chain(std::iter::once(0 as u8));

    for byte in prepared_bytes {
        if byte != last_byte {
            result.push(last_byte);
            if educational_mode {
                result.append(&mut private_utils::digits_from_byte(last_byte_counter));
            } else {
                result.push(last_byte_counter);
            }
            last_byte = byte;
            last_byte_counter = 1;
        } else {
            last_byte_counter += 1;
        }
    }

    result
}
