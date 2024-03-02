use std::collections::HashMap;

pub fn decompress(s: Vec<u8>, _verbose: bool) -> Result<Vec<u8>, String> {
    let alias_len = 1;
    let mut alias_bytes = Vec::<Vec<u8>>::new();
    for i in 1..32 {
        alias_bytes.push(vec![i]);
    } 
    let header_term = '\u{00}' as u8;

    // construct alias map
    let mut alias_map = HashMap::<Vec<u8>, Vec<u8>>::new();
    let mut bi = 0;
    let mut seq = Vec::<u8>::new();
    while bi < s.len() && s[bi] != header_term {
        let p_alias = &s[bi..(bi + alias_len)].to_vec(); // potential alias
        if alias_bytes.contains(&p_alias) {
            if alias_map.contains_key(p_alias) {
                // expand existing alias
                seq.extend(alias_map.get(p_alias).unwrap());
            } else {
                // add new alias
                alias_map.insert(
                    s[bi..(bi + alias_len)].to_owned(),
                    seq.clone()
                );
                seq.clear();
            }
            bi += alias_len;
            continue;
        }
        seq.push(s[bi]);
        bi += 1;
    }
    if bi >= s.len() {
        return Err("Header terminator not found".to_owned());
    }
    bi += 1;

    println!("Alias map: {:?}", alias_map);

    // build output string using alias map
    let mut decompressed = Vec::<u8>::new();
    while bi < s.len() {
        let p_alias = &s[bi..(bi + alias_len)]; // potential alias
        if alias_map.contains_key(p_alias) {
            decompressed.append(&mut (alias_map.get(p_alias).unwrap().to_owned()));
            bi += alias_len;
            continue;
        }
        decompressed.push(s[bi]);
        bi += 1;
    }

    Ok(decompressed)
}


