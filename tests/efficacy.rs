use std::fs;
use std::path::PathBuf;
use ztext::{compress::compress, decompress::decompress};

#[test]
fn compress_utf8() {
    let input_file = PathBuf::from("files/simple.txt");
    let input = match fs::read(input_file) {
        Ok(s) => s,
        Err(e) => {
            println!("error: {}", e);
            return;
        }
    };

    let res = compress(input.clone(), false);
    assert!(res.is_ok());
    assert!(res.unwrap().len() < input.len());
}

#[test]
fn decompress_utf8() {
    let input_file = PathBuf::from("files/simple.txt");
    let input = match fs::read(input_file) {
        Ok(s) => s,
        Err(e) => {
            println!("error: {}", e);
            return;
        }
    };

    let compressed = compress(input.clone(), true).unwrap().into_bytes();
    let res = decompress(compressed, false);
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), String::from_utf8(input).unwrap());
}
