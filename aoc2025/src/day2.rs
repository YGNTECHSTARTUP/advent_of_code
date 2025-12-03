use std::fs::read_to_string;

pub fn day2() {
    let a = read_to_string("puzzles/puz2.txt").unwrap();
    let mut p: Vec<i64> = Vec::new();

    let c: Vec<(i64, i64)> = a
        .split(',')
        .map(|x| {
            let d: Vec<&str> = x.trim().split('-').collect();
            (d[0].parse().unwrap(), d[1].parse().unwrap())
        })
        .collect();
    println!("{:?}", c);
    for i in c {
        for k in i.0..=i.1 {
            let mut found = false;
            let ch: Vec<char> = format!("{k}").chars().collect();
            let len = ch.len();
            if len < 2 {
                continue;
            }
            for pat in 1..=(len / 2) {
                if len % pat != 0 {
                    continue;
                }
                let mut ok = true;
                for d in 0..len {
                    if ch[d] != ch[d % pat] {
                        ok = false;
                        break;
                    }
                }

                if ok {
                    found = true;
                    break;
                }
            }
            if found {
                p.push(k);
            }
        }
    }

    let d: i64 = p.iter().sum();
    println!("{:?}", d);
}
