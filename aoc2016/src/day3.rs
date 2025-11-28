use std::fs::read_to_string;

pub fn day3() {
    let a = read_to_string("puzzles/puz3.txt").unwrap();
    let mut total_count = 0;
    let p: Vec<Vec<u32>> = a
        .lines()
        .map(|x| {
            x.trim()
                .split_whitespace()
                .map(|x| x.trim().parse().unwrap())
                .collect()
        })
        .collect();
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    for i in p {
        a.push(i[0]);
        b.push(i[1]);
        c.push(i[2]);
    }
    for sides in a.chunks_mut(3) {
        sides.sort();
        if sides[0] + sides[1] > sides[2] {
            total_count += 1;
        }
    }
    for sides in b.chunks_mut(3) {
        sides.sort();
        if sides[0] + sides[1] > sides[2] {
            total_count += 1;
        }
    }
    for sides in c.chunks_mut(3) {
        sides.sort();
        if sides[0] + sides[1] > sides[2] {
            total_count += 1;
        }
    }

    println!("{:?}", total_count);
}
