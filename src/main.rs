use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod compress;
mod decompress;

const EXTENSION: &str = "zt";

fn main() {
    let args: Vec<String> = env::args().collect(); 
    if args.len() < 2 {
        println!("Usage: {} [-d] [-v] <file>", args[0]);
        return;
    }

    let mut verbose = false;
    let mut decompress = false;
    let mut input_file = None;
    for arg in &args[1..] {
        println!("arg: {}", arg);
        match arg.as_str() {
            "-d" => decompress = true,
            "-v" => verbose = true,
            _ => input_file = Some(Path::new(arg)),
        }
    }
    if input_file.is_none() {
        println!("No input file specified");
        return;
    }
    let input_file = input_file.unwrap();

    let input_str = match fs::read_to_string(input_file) {
        Ok(s) => s[0..(s.len()-1)].to_string(),
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let size_orig = input_str.len();

    let output_str: String;
    let mut output_file = PathBuf::from(input_file.file_name().unwrap());
    if decompress {
        output_str = match decompress::decompress(&input_str, verbose) {
            Ok(s) => s, 
            Err(e) => {
                println!("Decompression failed: {}", e);
                return;
            }
        };
        output_file.set_extension(".txt");
        decompress_summary(&input_str, &output_str, size_orig, verbose);
    } else {
        output_str = match compress::compress(&input_str, verbose) {
            Ok(s) => s,
            Err(e) => {
                println!("Compression failed: {}", e);
                return;
            }
        };
        output_file.set_extension(EXTENSION);
        compress_summary(&input_str, &output_str, size_orig, verbose);
    }

    match fs::write(&output_file, output_str) {
        Ok(_) => println!("Output written to {}", output_file.to_str().unwrap()),
        Err(e) => println!("Error: {}", e),
    }
}

fn compress_summary(is: &str, os: &str, size_orig: usize, verbose: bool) {
    if verbose {
        println!("Original: {}", is);
        println!("Compressed: {}\n", os);
    }
    println!("Original size: {} bytes", size_orig);
    println!("Compressed size: {} bytes", os.len());
    println!("Compression ratio: {:.2}%", (os.len() as f64 / size_orig as f64) * 100.0);
}

fn decompress_summary(is: &str, os: &str, size_orig: usize, verbose: bool) {
    if verbose {
        println!("Original: {}", is);
        println!("Decompressed: {}\n", os);
    }
    println!("Original size: {} bytes", size_orig);
    println!("Decompressed size: {} bytes", os.len());
}
