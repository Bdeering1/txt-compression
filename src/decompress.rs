use std::collections::HashMap;
use std::str;

pub fn decompress(s: Vec<u8>, _verbose: bool) -> Result<String, String> {
    let alias_len = 1;
    let alias_chars = vec!["{", "}", "[", "]", "(", ")", "~", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "@", "#", "$", "%", "^", "&", "*", "_", "+", "="];
    let header_term = '|';

    // construct alias map
    let mut alias_map = HashMap::<Vec<u8>, Vec<u8>>::new();
    let mut ci = 0;
    let mut seq = Vec::<u8>::new();
    while ci < s.len() && s[ci] != header_term as u8 {
        let alias = &s[ci..(ci + alias_len)]; // potential alias
        if alias_chars.contains(&str::from_utf8(alias).unwrap()) {
            if alias_map.contains_key(alias) {
                // expand existing alias
                seq.extend(alias_map.get(alias).unwrap());
            } else {
                // add new alias
                alias_map.insert(
                    s[ci..(ci + alias_len)].to_owned(),
                    seq.clone()
                );
                seq.clear();
            }
            ci += alias_len;
            continue;
        }
        seq.push(s[ci]);
        ci += 1;
    }
    if ci >= s.len() {
        return Err("Header terminator not found".to_owned());
    }
    ci += 1;

    // build output string using alias map
    let mut decompressed = String::new();
    while ci < s.len() {
        let alias = &s[ci..(ci + alias_len)]; // potential alias
        if alias_map.contains_key(alias) {
            decompressed.push_str(&str::from_utf8(
                alias_map.get(alias).unwrap()
            ).unwrap());
            ci += alias_len;
            continue;
        }
        decompressed.push(s[ci] as char);
        ci += 1;
    }

    Ok(decompressed)
}
