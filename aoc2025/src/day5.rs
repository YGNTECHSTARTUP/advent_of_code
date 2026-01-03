use std::{collections::HashSet, fs::read_to_string};

// pub fn day5() {
//     let data = read_to_string("puzzles/puz5.txt").expect("Needed");
//     let sections: Vec<&str> = data.split("\n\r\n").collect();
//     let d: Vec<(u64, u64)> = sections[0]
//         .lines()
//         .map(|x| {
//             if x.is_empty() {
//                 return (0, 0);
//             }
//             // println!("{:?}", x);
//             let (l, r) = x.split_once('-').unwrap();

//             (l.parse().unwrap(), r.trim_end().parse().unwrap())
//         })
//         .collect();
//     let ans: usize = sections[1]
//         .lines()
//         .filter(|x| {
//             let x: u64 = x.trim_end().parse().unwrap();
//             d.iter().any(|p| x >= p.0 && x <= p.1)
//         })
//         .count();
//     println!("{:?}", ans);
// }

pub fn day5() {
    let data = read_to_string("puzzles/puz5.txt").expect("Needed");
    let sections: Vec<&str> = data.split("\n\r\n").collect();
    let mut d: Vec<(u64, u64)> = sections[0]
        .lines()
        .map(|x| {
            if x.is_empty() {
                return (0, 0);
            }
            // println!("{:?}", x);
            let (l, r) = x.split_once('-').unwrap();

            (l.parse().unwrap(), r.trim_end().parse().unwrap())
        })
        .collect();
    d.sort_by_key(|(l, _)| *l);
    let mut total = 0;
    let mut curr_start = d[0].0;
    let mut curr_end = d[1].1;
    for (l, r) in &d[1..] {
        if *l <= curr_end {
            curr_end = curr_end.max(*r);
        } else {
            total += curr_end - curr_start + 1;
            curr_start = *l;
            curr_end = *r;
        }
    }
    total += curr_end - curr_start + 1;
    println!("{:?}", total);
}
