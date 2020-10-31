mod utils;

mod lzw;
mod rle;
use std::error::Error;
// use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // let x = fs::read("./examples/Text_de_base.txt").expect("Couldn't read the file.");
    // println!("{}", x.len())

    // let z = Vec::from("one#two#rrrrrrea#yessssssssssssssssssssalutttttttt");
    let input = Vec::from("WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW");

    let code1 = rle::encode_with_delimiter(&input, false);
    let code1_edu = rle::encode_with_delimiter(&input, true);
    let code2 = rle::encode(&input, false);
    let code2_edu = rle::encode(&input, true);
    let decode = rle::decode_with_delimiter(&code1);

    println!(
        "Input length: {}
Code1 length: {}
Code1_edu length: {}, code1_edu: {}
Code2 length: {}
Code2 length: {}, code2: {}
Decode length: {}, decode: {}",
        input.len(),
        code1.len(),
        code1_edu.len(),
        String::from_utf8_lossy(&code1_edu[..]),
        code2.len(),
        code2_edu.len(),
        String::from_utf8_lossy(&code2_edu[..]),
        decode.len(),
        String::from_utf8_lossy(&decode[..])
    );

    let input2 = Vec::from("Bacchanales Bacchus");

    print!("\n\n");
    let (code, code32) = lzw::encode(&input2);
    println!(
        "
Input length: {}
Code length: {}, code:   {:?}
Code32 len:  {}, code32: {:?}
UTF code:   {}",
        input2.len(),
        code.len(),
        code,
        code32.len(),
        code32,
        String::from_utf8_lossy(&code[..])
    );

    // 66 97 99 99 104 97 110 97 108 101 115 32 256 258 104 117 115

    if lzw::hello() != 3 {
        return Ok(());
    }

    Ok(())
}
