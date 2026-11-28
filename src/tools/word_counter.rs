use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn character_count(source: &mut BufReader<File>) -> HashMap<u8, f64> {
    let mut m = HashMap::new();
    let mut total_char = 0;
    let mut invalid_char = 0;

    for line in source.lines() {
        let line = line.unwrap();
        for &b in line.as_bytes() {
            if !b.is_ascii_alphanumeric() && b != b' ' {
                invalid_char += 1;
                continue;
            }
            let mut key = b;
            if b.is_ascii_uppercase() {
                key = b.to_ascii_lowercase();
            }
            total_char += 1;
            *m.entry(key).or_insert(0) += 1;
        }
    }

    let mut rs = HashMap::new();
    for k in m.keys() {
        rs.insert(*k, *m.get(k).unwrap_or(&0) as f64 / total_char as f64);
        // println!(
        //     "{:?} = {}",
        //     char::from_u32(*k as u32).unwrap_or_default(),
        //     rs.get(k).unwrap_or(&0.0)
        // )
    }
    // println!("Invalid count: {}", invalid_char);
    // println!("Valid count: {}", total_char);
    rs
}

pub fn score_string(s: &[u8], expected_freq: &HashMap<u8, f64>) -> Option<f64> {
    let mut m = HashMap::new();
    let mut score = 0.0;
    for &b in s {
        if b > 127 {
            return None;
        }
        if !b.is_ascii_alphanumeric() && b != b' ' {
            score += 1.0;
            continue;
        }
        let mut key = b;
        if b.is_ascii_uppercase() {
            key = b.to_ascii_lowercase();
        }
        *m.entry(key).or_insert(0) += 1;
    }
    for k in m.keys() {
        let freq = *m.get(k).unwrap_or(&0) as f64 / s.len() as f64;
        let expected = expected_freq.get(k).unwrap_or(&1.0);
        score += (freq - expected) * (freq - expected) / expected;
    }

    Some(score)
}
