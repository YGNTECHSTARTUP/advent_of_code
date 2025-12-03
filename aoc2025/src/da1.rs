use std::fs::read_to_string;

pub fn day1_part2() {
    let input = read_to_string("puzzles/puz1.txt").unwrap();
    let mut pos = 50;
    let mut hits = 0;

    for line in input.lines() {
        let (dir, rest) = line.split_at(1);
        let steps: i32 = rest.parse().unwrap();

        if dir == "R" {
            for _ in 0..steps {
                pos = (pos + 1) % 100;
                if pos == 0 {
                    hits += 1;
                }
            }
        } else if dir == "L" {
            for _ in 0..steps {
                pos -= 1;
                if pos < 0 {
                    pos = 99;
                }
                if pos == 0 {
                    hits += 1;
                }
            }
        }
    }

    println!("Password = {}", hits);
}

pub fn day1_part1() {
    let input = read_to_string("puzzles/puz1.txt").unwrap();
    let mut pos = 50;
    let mut hits = 0;
    for line in input.lines() {
        let (dir, rest) = line.split_at(1);
        let steps: i32 = rest.parse().unwrap();

        if dir == "R" {
            pos = (pos + steps) % 100;
        } else if dir == "L" {
            pos = ((pos - steps) % 100 + 100) % 100;
        }
        if pos == 0 {
            hits += 1;
        }
    }
    println!("{:?}", hits);
}
