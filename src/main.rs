mod rle;
// use std::fs;

fn main() {
    // let x = fs::read("./examples/Text_de_base.txt").expect("Couldn't read the file.");
    // println!("{}", x.len())

    // let z = Vec::from("one#two#rrrrrrea#yessssssssssssssssssssalutttttttt");
    let input = Vec::from("WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW");

    let code1 = rle::encode_with_delimiter(&input, b'#');
    let code2 = rle::encode(&input);
    println!(
        "Input length: {}\nCode1 length: {}, code1: {}\nCode2 length: {}, code2: {}",
        input.len(),
        code1.len(),
        String::from_utf8_lossy(&code1[..]),
        code2.len(),
        String::from_utf8_lossy(&code2[..])
    );
}
