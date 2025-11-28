use std::{collections::HashSet, fs::read_to_string};

pub fn day1() {
    let a = read_to_string("puzzles/puz1.txt").unwrap();
    let k: Vec<(&str, i32)> = a
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| {
            let k = x.trim().split_at(1);
            let k: (&str, i32) = (k.0, k.1.parse().unwrap());
            k
        })
        .collect();
    let mut dir = 0;
    let mut coord = (0, 0);
    let mut dir_coord = (0, 0);
    let mut hs = HashSet::new();
    hs.insert(coord);
    for i in k {
        match i.0 {
            "L" => {
                dir = (dir + 3) % 4;
            }
            "R" => {
                dir = (dir + 1) % 4;
            }
            _ => {}
        }
        dir_coord = match dir {
            0 => (0, 1),
            1 => (1, 0),
            2 => (0, -1),
            3 => (-1, 0),
            _ => (0, 0),
        };
        for _ in 0..i.1 {
            coord.0 += dir_coord.0;
            coord.1 += dir_coord.1;
            if !hs.insert(coord) {
                println!("{:?}", coord);
                break;
            }
        }
    }
}
