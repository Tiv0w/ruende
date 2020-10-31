mod utils;

mod lzw;
mod rle;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:#?}", args);

    if args.len() < 3 {
        println!("Usage: ruende FILE DEST");
        std::process::exit(64);
    }

    let x = fs::read(&args[1]).expect("Couldn't read the file.");

    let code16 = lzw::v2::encode(&x);

    let compression_ratio = code16.len() as f32 / x.len() as f32;

    println!(
        "
SHAKESPEARE:

Text   length: {}
Code16 length: {}
Space gain       : {:.1}% ({})
Compression ratio: {:.1}% ({})
Space saving     : {:.1}% ({})
",
        x.len(),
        code16.len(),
        compression_ratio * 100 as f32,
        compression_ratio,
        100 as f32 / compression_ratio,
        1 as f32 / compression_ratio,
        (1 as f32 - compression_ratio) * 100 as f32,
        1 as f32 / compression_ratio
    );

    fs::write(&args[2], code16).expect("Couldn't write to DEST");
}
