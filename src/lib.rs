use std::path::PathBuf;
use std::fs;

pub mod compress;
pub mod decompress;

const COMPRESSED_EXT: &str = "zt";
const DECOMPRESSED_EXT: &str = "txt";

pub fn compress_file(bytes: Vec<u8>, file_stem: &str, extension: Option<&str>, verbose: bool) -> Result<(), String> {
    let input_len = bytes.len();
    let output_str = match compress::compress(bytes, verbose) {
        Ok(s) => s,
        Err(e) => {
            return Err(format!("Compression failed: {}", e))
        }
    };
    let mut output_file = PathBuf::from(file_stem);
    if let Some(ext) = extension {
        output_file.set_extension(ext);
    } else {
        output_file.set_extension(COMPRESSED_EXT);
    }
    match write_output(&output_file, &output_str) {
        Ok(s) => println!("{}", s),
        Err(e) => return Err(e),
    }

    println!("Original size: {} bytes", input_len);
    println!("Compressed size: {} bytes", output_str.len());
    println!("Compression ratio: {:.2}%", (output_str.len() as f64 / input_len as f64) * 100.0);
    Ok(())
}

pub fn decompress_file(bytes: Vec<u8>, file_stem: &str, extension: Option<&str>, verbose: bool) -> Result<(), String> {
    let input_len = bytes.len();
    let output_str = match decompress::decompress(bytes, verbose) {
        Ok(s) => s, 
        Err(e) => {
            return Err(format!("Decompression failed: {}", e))
        }
    };
    let mut output_file = PathBuf::from(file_stem);
    if let Some(ext) = extension {
        output_file.set_extension(ext);
    } else {
        output_file.set_extension(DECOMPRESSED_EXT);
    }
    match write_output(&output_file, &output_str) {
        Ok(s) => println!("{}", s),
        Err(e) => return Err(e),
    }

    println!("Original size: {} bytes", input_len);
    println!("Decompressed size: {} bytes", output_str.len());
    Ok(())
}

fn write_output(output_file: &PathBuf, output_str: &str) -> Result<String, String> {
    match fs::write(&output_file, output_str) {
        Ok(_) => Ok(format!("Output written to {}", output_file.to_str().unwrap())),
        Err(e) => Err(format!("Failed to write output: {}", e)),
    }
}
