mod compress;

fn main() {
    let sample_str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";
    let size_orig = sample_str.len();
    println!("Input string: {}", sample_str);
    match compress::compress(sample_str) {
        Ok(s) => {
            println!("Compressed string: {}", s);
            println!("Original size: {} bytes", size_orig);
            println!("Compressed size: {} bytes", s.len());
            println!("Compression ratio: {:.2}%", (s.len() as f64 / size_orig as f64) * 100.0);
        }
        Err(e) => println!("Compression failed: {}", e),
    }
}
