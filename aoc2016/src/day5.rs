use std::char;

use hex::encode;
use md5::compute;
pub fn day5() {
    let mut i = 0;
    let mut password: Vec<char> = vec!['_'; 8];
    let mut max_count = 8;
    loop {
        let d = compute(format!("reyedfim{i}"));

        let k = encode(*d);
        if k.starts_with("00000") {
            println!("{:?}", i);
            let k = k.chars().collect::<Vec<char>>();
            let mut pos: usize = 0;
            match k[5].to_string().parse() {
                Ok(d) => {
                    pos = d;
                }
                Err(e) => {
                    i += 1;
                    continue;
                }
            };

            let chare = k[6];
            if pos >= 8 {
                i += 1;
                continue;
            }
            let d = password.get(pos).unwrap();
            if *d == '_' {
                password[pos] = chare;
                max_count -= 1;

                println!("{:?}", password);
            }
        }
        if max_count == 0 {
            break;
        }
        i += 1;
    }
    println!("{:?}", password);
}
