use std::collections::HashMap;

pub fn compress(mut s: Vec<u8>, verbose: bool) -> Result<Vec<u8>, String> {
    let alias_len = 1;

    let mut alias_bytes = find_alias_bytes(&s)
        .iter()
        .map(|b| vec![*b])
        .collect::<Vec<Vec<u8>>>();
    let null_byte = '\u{00}' as u8;

    let mut patterns = find_patterns(&s, alias_len);
    patterns.sort_by(|a, b| {
        a.chars.len().cmp(&b.chars.len())
    });
    patterns.retain(|p| p.savings(alias_len) > 0);
    //print_patterns(&patterns, alias_len);

    if patterns.len() == 0 {
        return Err("Unable to compress input.".to_string());
    }

    let mut alias_mappings = Vec::<AliasEntry>::new();
    let mut bi;
    loop {
        // get next longest sequence with positive savings
        let mut p = match patterns.pop() {
            Some(p) => p,
            None => break
        };
        p.count = 0;
        bi = 0;
        while bi < s.len() {
            if bi + p.chars.len() <= s.len() && &s[bi..(bi + p.chars.len())] == p.chars {
                p.count += 1;
                bi += p.chars.len();
                continue;
            }
            bi += 1;
        }
        if p.savings(alias_len) <= 0 {
            continue;
        }

        // assign alias
        let alias = alias_bytes.pop();
        let alias = match alias {
            Some(a) => a,
            None => break
        };
        alias_mappings.push(AliasEntry {
            bytes: p.chars.to_owned(),
            alias: alias.clone()
        });

        if verbose {
            println!("Replacing {:?} count: {} savings: {}",
                String::from_utf8(p.chars.to_owned()).unwrap(),
                p.count,
                p.savings(alias_len)
            );
        }

        // replace all instances with alias
        bi = 0;
        while bi < s.len() {
            if bi + p.chars.len() <= s.len() && &s[bi..(bi + p.chars.len())] == p.chars {
                // mark alias position in byte string
                let mut a_idx = 0;
                for b in &alias {
                    s[bi + a_idx] = *b;
                    a_idx += 1;
                }
                // replace remaining chars with null char
                for pi in a_idx..p.chars.len() {
                    s[bi + pi] = null_byte;
                }
                bi += p.chars.len();
                continue;
            }
            bi += 1;
        }
    }

    // push aliases to string, shortest first (allowing smaller aliases to be used within the header)
    let mut compressed = Vec::<u8>::new();
    let mut written_aliases = Vec::<AliasEntry>::new();
    alias_mappings.sort_by(|a, b| { b.bytes.len().cmp(&a.bytes.len()) });
    while let Some(mut a) = alias_mappings.pop()  {
        for wa in &written_aliases {
            let mut bi = 0;
            while bi < a.bytes.len() {
                if bi + wa.bytes.len() <= a.bytes.len() && &a.bytes[bi..(bi + wa.bytes.len())] == wa.bytes {
                    let mut wa_idx = 0;
                    for b in &wa.alias {
                        a.bytes[bi + wa_idx] = *b;
                        wa_idx += 1;
                    }
                    for i in wa_idx..wa.bytes.len() {
                        a.bytes[bi + i] = null_byte;
                    }
                }
                bi += 1;
            }
        }
        // write aliased sequence
        for b in &a.bytes {
            if *b != null_byte {
                compressed.push(*b);
            }
        }
        // write alias
        for b in &a.alias {
            if *b != null_byte {
                compressed.push(*b);
            }
        }
        written_aliases.push(a);
    }
    compressed.push(null_byte); // header term

    // push remaining bytes to string
    for b in &s {
        if *b != null_byte {
            compressed.push(*b);
        }
    }
    if compressed.len() >= s.len() {
        return Err("Unable to compress input.".to_string());
    }

    Ok(compressed)
}

fn find_patterns(s: &Vec<u8>, alias_len: usize) -> Vec<Pattern> {
    let mut p_map = HashMap::<Vec<u8>, usize>::new();

    // many windows larger than 1/3 of the input have no possible matches
    // we assume repeated sequences of size n/3 < m < n/2 are very rare
    let mut pattern_size = s.len() / 3;
    while pattern_size > alias_len {
        // println!("Pattern size: {}", pattern_size);
        for seq in s.windows(pattern_size) {
            // println!("Window: {:?}", String::from_utf8(c.to_vec()).unwrap());
            if p_map.contains_key(seq) {
                *p_map.get_mut(seq).unwrap() += 1;
            } else {
                p_map.insert(seq.to_owned(), 1);
            }
        }
        pattern_size -= 1;
    }

    let mut patterns = Vec::new();
    for (k, v) in p_map.iter() {
        patterns.push(Pattern::new(k.to_owned(), *v));
    }

    patterns
}

struct AliasEntry {
    bytes: Vec<u8>,
    alias: Vec<u8>,
}

struct Pattern {
    chars: Vec<u8>,
    count: usize,
} 

impl Pattern {
    fn new(chars: Vec<u8>, count: usize) -> Pattern {
        Pattern {
            chars,
            count,
        }
    }

    fn savings(&self, alias_len: usize) -> i32 {
        let len = self.chars.len() as i32;
        let reps = self.count as i32;
        let alias_len = alias_len as i32;
        len * reps - (reps + 1) * alias_len - len
    }
}

fn find_alias_bytes(s: &Vec<u8>) -> Vec<u8> {
    let mut used_bytes = vec![false; 256];
    for c in s {
        used_bytes[*c as usize] = true;
    }

    let mut available_bytes = Vec::<u8>::new();
    for i in (1..32 /*256*/).rev() { // up to 256 can be used if bytes 32+ are specified in header
        if !used_bytes[i] {
            available_bytes.push(i as u8);
        } 
    }

    // print!("Available alias bytes: ");
    // for ab in &available_bytes {
    //     print!("{:?}, ", ab);
    // }
    // println!();

    available_bytes
}
