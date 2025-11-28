use std::{collections::HashMap, fs::read_to_string};

pub fn day4() {
    let a = read_to_string("puzzles/puz4.txt").unwrap();
    let lines: Vec<&str> = a.lines().collect();

    let mut rooms: Vec<(HashMap<char, u32>, u32, String, String)> = Vec::new();

    for line in lines {
        let mut sector_digits = Vec::new();
        let mut letters = Vec::new();
        let mut checksum_chars = Vec::new();
        let mut bracket_idx = 0;

        for (idx, ch) in line.chars().enumerate() {
            if ch == '-' {
                continue;
            }
            if ch.is_ascii_digit() {
                sector_digits.push(ch);
                continue;
            }
            if ch == '[' {
                bracket_idx = idx;
                break;
            }
            letters.push(ch);
        }

        let (_, right) = line.split_at(bracket_idx);
        for ch in right.chars() {
            if ch == ']' {
                break;
            } else if ch == '[' {
                continue;
            }
            checksum_chars.push(ch);
        }

        let checksum: String = checksum_chars.iter().collect();

        let mut freq = HashMap::new();
        for ch in letters.iter() {
            *freq.entry(*ch).or_insert(0) += 1;
        }

        let sector_id: u32 = sector_digits.iter().collect::<String>().parse().unwrap();

        rooms.push((freq, sector_id, checksum, letters.iter().collect()));
    }

    let mut sum_sector_ids = 0;

    for (freq, sector_id, checksum, l) in rooms {
        // Turn HashMap into Vec for sorting
        let mut items: Vec<(char, u32)> = freq.into_iter().collect();

        // Sort by:
        // 1) descending count
        // 2) ascending char (for ties)
        items.sort_by(|a, b| {
            b.1.cmp(&a.1) // desc by frequency
                .then_with(|| a.0.cmp(&b.0)) // asc by character
        });

        // Take first 5 chars as computed checksum
        let computed: String = items.iter().take(5).map(|(ch, _)| *ch).collect();

        if computed == checksum {
            let r = sector_id % 26;
            if decry(&l, r).contains(&"north") {
                println!("{:?}", sector_id);
                break;
            }
        }
    }

    // println!("Sum of valid sector IDs: {}", sum_sector_ids);
}

pub fn decry(tx: &str, r: u32) -> String {
    let k: Vec<char> = tx.chars().collect();
    let d: String = k
        .iter()
        .map(|x| {
            if *x == '-' {
                ' ' as char
            } else if x.is_lowercase() {
                let base = b'a';
                let d = ((*x as u8 - base + r as u8) % 26) + base;
                d as char
            } else {
                *x
            }
        })
        .collect();
    d
}
