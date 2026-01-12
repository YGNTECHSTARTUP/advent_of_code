use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

pub fn day7() {
    let a = read_to_string("puzzles/puz7.txt").expect("A valid file name");
    let matrix: Vec<Vec<char>> = a.lines().map(|x| x.chars().collect()).collect();
    let mut seen = HashSet::new();
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut active = HashSet::new();
    let mut idx = 0;
    let mut hse = HashMap::new();

    for i in 0..matrix[0].len() {
        if matrix[0][i] == 'S' {
            idx = i;
            break;
        }
    }
    let timeline_count = timeline(&matrix, 1, idx as isize, &mut hse);
    active.insert((1, idx));
    let mut split_count = 0;
    while !active.is_empty() {
        let mut next_active = HashSet::new();
        for (r, c) in active {
            if r >= rows || c >= cols || !seen.insert((r, c)) {
                continue;
            }
            match matrix[r][c] {
                '.' => {
                    next_active.insert((r + 1, c));
                }
                '^' => {
                    split_count += 1;
                    if c > 0 {
                        next_active.insert((r + 1, c - 1));
                    }
                    next_active.insert((r + 1, c + 1));
                }
                _ => {}
            }
        }
        active = next_active;
    }
    println!("{:?}", split_count);
    println!("{:?}", timeline_count);
}

fn timeline(
    grid: &Vec<Vec<char>>,
    r: isize,
    c: isize,
    memo: &mut HashMap<(isize, isize), u64>,
) -> u64 {
    if r >= grid.len() as isize || c < 0 || c >= grid[r as usize].len() as isize {
        return 1;
    }

    if let Some(&v) = memo.get(&(r, c)) {
        return v;
    }

    let result = match grid[r as usize][c as usize] {
        '.' | 'S' => timeline(grid, r + 1, c, memo),
        '^' => timeline(grid, r + 1, c - 1, memo) + timeline(grid, r + 1, c + 1, memo),
        _ => 0,
    };

    memo.insert((r, c), result);
    result
}
