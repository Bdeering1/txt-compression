use std::collections::HashMap;

#[allow(unused_variables)]
pub fn compress(s: &str) -> Result<String, String> {
    let mut s = s.as_bytes().to_vec();
    let alias_len = 1;

    let mut alias_chars = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "@", "#", "$", "%", "&", "*", "_", "+", "="];
    let header_term = '|';
    let null_char = '0';

    let mut patterns = find_patterns(&s, alias_len);
    patterns.sort_by(|a, b| {
        a.savings(alias_len).cmp(&b.savings(alias_len))
    });
    patterns.retain(|p| p.savings(alias_len) > 0);
    print_patterns(&patterns, alias_len);

    if patterns.len() == 0 {
        return Err("Unable to compress input.".to_string());
    }

    let mut compressed = String::new();
    let mut ci;
    loop {
        // get sequence with most savings
        let mut p = match patterns.pop() {
            Some(p) => p,
            None => break
        };
        p.count = 0;
        ci = 0;
        while ci < s.len() {
            if ci + p.chars.len() <= s.len() && &s[ci..(ci + p.chars.len())] == p.chars {
                p.count += 1;
                ci += p.chars.len();
                continue;
            }
            ci += 1;
        }
        if p.savings(alias_len) <= 0 {
            println!("Discarding {:?} count: {} savings: {}",
                String::from_utf8(p.chars.to_owned()).unwrap(),
                p.count,
                p.savings(alias_len)
            );
            continue;
        }
        println!("Replacing {:?} count: {} savings: {}",
            String::from_utf8(p.chars.to_owned()).unwrap(),
            p.count,
            p.savings(alias_len)
        );

        // assign alias
        if p.alias.is_none() {
            p.alias = match alias_chars.pop() {
                Some(seq) => Some(seq.to_owned()),
                None => break
            };
            compressed.push_str(&p.alias.as_ref().unwrap());
            compressed.push_str(&String::from_utf8(p.chars.to_owned()).unwrap());
        }

        // replace all instances with alias
        ci = 0;
        while ci < s.len() {
            if ci + p.chars.len() <= s.len() && &s[ci..(ci + p.chars.len())] == p.chars {
                // write alias to byte string
                let mut a_idx = 0;
                for c in p.alias.as_ref().unwrap().as_bytes() {
                    s[ci + a_idx] = *c;
                    a_idx += 1;
                }
                // replace remaining chars with null char
                for pi in a_idx..p.chars.len() {
                    s[ci + pi] = null_char as u8;
                }
                ci += p.chars.len();
                continue;
            }
            ci += 1;
        }
    }
    compressed.push(header_term);
    for c in s {
        if c != null_char as u8 {
            compressed.push(c as char);
        }
    }

    return Ok(compressed);
}

fn find_patterns(s: &Vec<u8>, alias_len: usize) -> Vec<Pattern> {
    let mut p_map = HashMap::new();

    // many windows larger than 1/3 of the input have no possible matches
    // we assume repeated sequences of size n/3 < m < n/2 are very rare
    let mut pattern_size = s.len() / 3;
    while pattern_size > alias_len {
        // println!("Pattern size: {}", pattern_size);
        for c in s.windows(pattern_size) {
            // println!("Window: {:?}", String::from_utf8(c.to_vec()).unwrap());
            if p_map.contains_key(c) {
                *p_map.get_mut(c).unwrap() += 1;
            } else {
                p_map.insert(c.to_owned(), 1);
            }
        }
        pattern_size -= 1;
    }

    let mut patterns = Vec::new();
    for (k, v) in p_map.iter() {
        patterns.push(Pattern::new(k.to_owned(), *v));
    }

    return patterns;
}

fn print_patterns(patterns: &Vec<Pattern>, alias_len: usize) {
    for p in patterns {
        println!("found {:?} {} times ({} bytes reducable)",
            String::from_utf8(p.chars.to_vec()).unwrap(),
            p.count,
            p.savings(alias_len)
        );
    }
}

struct Pattern {
    chars: Vec<u8>,
    count: usize,
    alias: Option<String>
} 

impl Pattern {
    fn new(chars: Vec<u8>, count: usize) -> Pattern {
        Pattern {
            chars,
            count,
            alias: None
        }
    }

    fn savings(&self, alias_len: usize) -> i32 {
        let len = self.chars.len() as i32;
        let reps = self.count as i32;
        let alias_len = alias_len as i32;
        len * reps - (reps + 1) * alias_len - len
    }
}
