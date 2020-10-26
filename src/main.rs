mod rlc;
use std::fs;

fn main() {
    println!("Hello, world!");
    let x = fs::read("./examples/Text_de_base.txt").expect("Couldn't read the file.");
    println!("{}", x.len());

    let z = Vec::from("one#two#rrrrrrea#yesssssssssssssss");
    println!("{}: {:?}", z.len(), z);

    let proc = rlc::utils::escape_delimiter(&z, '#');
    println!(
        "{}: {:?}\n{}",
        proc.len(),
        proc,
        String::from_utf8_lossy(&proc[..])
    );
}
