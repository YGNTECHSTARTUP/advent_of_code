use itertools::Itertools;
use std::fs::read_to_string;

// pub fn day3() {
//     let input = read_to_string("puzzles/puz3.txt").unwrap();
//     let mut total_power = 0u32;

//     for line in input.lines() {
//         let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
//         let mut best = 0u32;

//         for i in 0..digits.len() {
//             for j in i + 1..digits.len() {
//                 let value = digits[i] * 10 + digits[j];
//                 best = best.max(value);
//             }
//         }
//         total_power += best;
//     }

//     println!("{total_power}");
// }

pub fn day3() {
    let input = read_to_string("puzzles/puz3.txt").unwrap();
    let total: u64 = input
        .lines()
        .map(|bank| {
            let mut current_index = 0;
            let mut batteries = Vec::new();
            for i in 0..11 {
                let (index, first_max) = &bank[current_index..(bank.len() - 11 + i)]
                    .chars()
                    .enumerate()
                    .max_set_by_key(|(_index, battery)| *battery)
                    .first()
                    .unwrap()
                    .clone();
                batteries.push(*first_max);
                current_index = current_index + index + 1;
            }

            let (second_index, secound_max) = &bank[current_index..]
                .chars()
                .enumerate()
                .max_by_key(|(_index, battery)| *battery)
                .unwrap();
            batteries.push(*secound_max);
            batteries.iter().collect::<String>().parse::<u64>().unwrap()
        })
        .sum();
    println!("{:?}", total);
}
