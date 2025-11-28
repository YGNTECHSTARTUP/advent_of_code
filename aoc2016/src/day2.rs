use core::panic;
use std::{collections::HashMap, fs::read_to_string};

pub fn day2() {
    let a = read_to_string("puzzles/puz2.txt").unwrap();
    let p: Vec<Vec<char>> = a.lines().map(|x| x.chars().collect()).collect();
    let mut init: (u32, u32) = (1, 1);
    let mut ans = Vec::new();
    for i in p {
        for j in i {
            match j {
                'U' => {
                    if init.1 > 0 {
                        init.1 -= 1;
                    }
                }
                'D' => {
                    if init.1 < 2 {
                        init.1 += 1;
                    }
                }
                'L' => {
                    if init.0 > 0 {
                        init.0 -= 1;
                    }
                }
                'R' => {
                    if init.0 < 2 {
                        init.0 += 1;
                    }
                }
                _ => {
                    panic!("Unwanted commmand!");
                }
            };
        }
        let digit = 1 + init.0 + init.1 * 3;
        ans.push(digit);
    }
    println!("{:?}", ans);
}

pub fn day2_part2() {
    let mut hs: HashMap<(i32, i32), &str> = HashMap::new();
    hs.insert((-1, -1), "2");
    hs.insert((1, -1), "4");
    hs.insert((-1, 1), "A");
    hs.insert((1, 1), "C");
    hs.insert((0, 0), "7");
    hs.insert((1, 0), "8");
    hs.insert((2, 0), "9");
    hs.insert((-1, 0), "6");
    hs.insert((-2, 0), "5");
    hs.insert((0, 1), "B");
    hs.insert((0, 2), "D");
    hs.insert((0, -1), "3");
    hs.insert((0, -2), "1");
    let a = read_to_string("puzzles/puz2.txt").unwrap();
    let p: Vec<Vec<char>> = a.lines().map(|x| x.chars().collect()).collect();
    let mut init = (-2, 0);
    let mut res = Vec::new();
    for i in p {
        for j in i {
            match j {
                'U' => {
                    if hs.contains_key(&(init.0, init.1 - 1)) {
                        init.1 -= 1;
                    }
                }
                'D' => {
                    if hs.contains_key(&(init.0, init.1 + 1)) {
                        init.1 += 1;
                    }
                }
                'L' => {
                    if hs.contains_key(&(init.0 - 1, init.1)) {
                        init.0 -= 1;
                    }
                }
                'R' => {
                    if hs.contains_key(&(init.0 + 1, init.1)) {
                        init.0 += 1;
                    }
                }
                _ => {
                    panic!("Unwanted commmand!");
                }
            };
        }
        res.push(hs.get(&init).unwrap());
    }
    println!("{:?}", res);
}
