use std::{collections::HashSet, fs::read_to_string};

use glam::IVec2;

const NEIGHTBORS: [IVec2; 8] = [
    IVec2::X,
    IVec2::Y,
    IVec2::NEG_X,
    IVec2::NEG_Y,
    IVec2::ONE,
    IVec2::NEG_ONE,
    IVec2::new(1, -1),
    IVec2::new(-1, 1),
];

pub fn day4() {
    let data = read_to_string("puzzles/puz4.txt").expect("Valid File Name");
    let mut pos = data
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, ch)| (ch == '@').then_some(IVec2::new(x as i32, y as i32)))
        })
        .collect::<HashSet<IVec2>>();
    println!("{:?}", pos);
    let count = pos
        .iter()
        .filter(|posi| {
            NEIGHTBORS
                .iter()
                .filter(|&off| pos.contains(&(*posi + off)))
                .count()
                < 4
        })
        .count();
    let mut removed_count = 0;
    loop {
        let rolls_to_remove: HashSet<IVec2> = pos
            .iter()
            .filter(|&posi| {
                NEIGHTBORS
                    .iter()
                    .filter(|&off| pos.contains(&(posi + *off)))
                    .count()
                    < 4
            })
            .cloned()
            .collect();
        if rolls_to_remove.len() == 0 {
            break;
        }
        removed_count += rolls_to_remove.len();
        pos = pos.difference(&rolls_to_remove).cloned().collect();
    }
    println!("{:?}", removed_count);
}
