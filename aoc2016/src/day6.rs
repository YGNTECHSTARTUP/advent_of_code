use std::{collections::HashMap, fs::read_to_string};

pub fn day6() {
    let a = read_to_string("puzzles/puz6.txt").unwrap();
    let d: Vec<Vec<char>> = a.lines().map(|x| x.chars().collect()).collect();
    let mut k = vec![
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];
    for i in d {
        for (d, j) in k.iter_mut().enumerate() {
            j.push(i[d]);
        }
    }
    println!("{:?}", k);
    let mut res = Vec::new();
    for i in k {
        let mut hs = HashMap::new();
        for j in i {
            *hs.entry(j).or_insert(0) += 1;
        }

        let mut c: Vec<(&char, &i32)> = hs.iter().collect();
        c.sort_by(|a, b| a.1.cmp(&b.1));
        println!("{:?}", c);
        res.push(c[0].0.clone());
    }
    println!("{:?}", res);
}
