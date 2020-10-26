pub mod utils {
    /// Escapes the given delimiter in data.
    pub fn escape_delimiter(data: &Vec<u8>, delimiter: char) -> Vec<u8> {
        let mut processed_data: Vec<u8> = Vec::new();

        for byte in data.iter() {
            if *byte == (delimiter as u8) {
                processed_data.push(delimiter as u8);
                processed_data.push(delimiter as u8);
            } else {
                processed_data.push(*byte);
            }
        }

        processed_data
    }
}
