use std::env;
use std::fs;
use std::path::Path;

use ztext::{compress_file, decompress_file};

fn main() {
    let args: Vec<String> = env::args().collect(); 
    if args.len() < 2 {
        println!("Usage: {} [-d] [-v] <file>", args[0]);
        return;
    }

    let mut verbose = false;
    let mut compress = true;
    let mut input_file = None;
    for arg in &args[1..] {
        match arg.as_str() {
            "-d" => compress = false,
            "-v" => verbose = true,
            _ => input_file = Some(Path::new(arg)),
        }
    }
    if input_file.is_none() {
        println!("No input file specified");
        return;
    }
    let input_file = input_file.unwrap();

    let input = match fs::read(input_file) {
        Ok(s) => s,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let file_stem = input_file.file_stem().unwrap().to_str().unwrap();
    if compress {
        if let Err(e) = compress_file(input, file_stem, None, verbose) {
            println!("{}", e);
        }
    } else {
        if let Err(e) = decompress_file(input, file_stem, None, verbose) {
            println!("{}", e);
        }
    }
}
