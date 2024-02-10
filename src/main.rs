use std::env;
use std::fs;

mod compress;
mod decompress;

fn main() {
    let args: Vec<String> = env::args().collect(); 
    if args.len() < 2 {
        println!("Usage: {} [-d] [-v] <file>", args[0]);
        return;
    }

    let mut verbose = false;
    let mut decompress = false;
    let mut file = "";
    for arg in &args[1..] {
        println!("arg: {}", arg);
        match arg.as_str() {
            "-d" => decompress = true,
            "-v" => verbose = true,
            _ => file = arg,
        }
    }

    let input_str = match fs::read_to_string(file) {
        Ok(s) => s[0..(s.len()-1)].to_string(),
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let size_orig = input_str.len();
    if verbose {
        println!("Original: {}", input_str);
    }
    if decompress {
        match decompress::decompress(file, verbose) {
            Ok(s) => decompress_summary(&s, size_orig, verbose),
            Err(e) => println!("Decompression failed: {}", e),
        }
    } else {
        match compress::compress(&input_str, verbose) {
            Ok(s) => compress_summary(&s, size_orig, verbose),
            Err(e) => println!("Compression failed: {}", e),
        }
    }

}

fn compress_summary(s: &str, size_orig: usize, verbose: bool) {
    if verbose {
        println!("Compressed: {}", s);
    }
    println!("Original size: {} bytes", size_orig);
    println!("Compressed size: {} bytes", s.len());
    println!("Compression ratio: {:.2}%", (s.len() as f64 / size_orig as f64) * 100.0);
}

fn decompress_summary(s: &str, size_orig: usize, verbose: bool) {
    if verbose {
        println!("Decompressed: {}", s);
    }
    println!("Original size: {} bytes", size_orig);
    println!("Decompressed size: {} bytes", s.len());
}
