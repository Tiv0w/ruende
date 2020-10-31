mod utils;

mod lzw;
mod rle;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: ruende FILE DEST");
        std::process::exit(64);
    }

    println!("Text: {}", &args[1]);

    let file = fs::read(&args[1]).expect("Couldn't read the file.");

    let code = lzw::v2::encode(&file);

    let compression_ratio = code.len() as f32 / file.len() as f32;

    println!(
        "
Text length      : {}
Code length      : {}
% of the original: {:.1}% ({})
Compression ratio: {:.1}% ({})
Space saved      : {:.1}% ({})
",
        file.len(),
        code.len(),
        compression_ratio * 100 as f32,
        compression_ratio,
        100 as f32 / compression_ratio,
        1 as f32 / compression_ratio,
        (1 as f32 - compression_ratio) * 100 as f32,
        1 as f32 - compression_ratio
    );

    fs::write(&args[2], code).expect("Couldn't write to DEST");
}
