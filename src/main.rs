#![allow(dead_code)]
extern crate clap;
use clap::{App, Arg};
use std::fs;

mod lzw;
mod rle;
mod utils;

fn main() {
    let matches = App::new("RUENDE")
        .version("0.1.0")
        .author("Tiv0w <t.theomeyer@gmail.com>")
        .about("RUst ENcoder & DEcoder is a general-purpose compression system.")
        .arg(
            Arg::with_name("decode")
                .short("d")
                .long("decode")
                .help("Tells RUENDE to decode INPUT"),
        )
        .arg(
            Arg::with_name("SRC")
                .help("Sets the source file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("DEST")
                .help("Sets the destination file")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("rle")
                .short("r")
                .long("rle")
                .help("Tells RUENDE to use RLE"),
        )
        .get_matches();

    let arg_src = matches
        .value_of("SRC")
        .expect("SRC argument cannot be read");
    let arg_dest = matches
        .value_of("DEST")
        .expect("DEST argument cannot be read");
    let arg_decode = matches.is_present("decode");

    if matches.args.len() < 2 {
        println!("{}", matches.usage());
        std::process::exit(64);
    }

    let file = fs::read(arg_src).expect("Couldn't read the file.");

    if !arg_decode {
        let code = lzw::v2::encode(&file);

        utils::compression::print_compression_ratio(file.len(), code.len());

        fs::write(arg_dest, code).expect("Couldn't write to DEST");
    } else {
        // let _decode = lzw::v2::decode(&code);
        let decode = lzw::v2::decode(&file);
        fs::write(arg_dest, decode).expect("Couldn't write to DEST");
    }
}
