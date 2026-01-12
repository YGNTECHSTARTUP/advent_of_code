use itertools::Itertools;
use std::fs::read_to_string;

pub fn day10() {
    let data = read_to_string("puzzles/puz10.txt").expect("Needed");

    let mut conf = Vec::new();
    let mut buttons = Vec::new();
    let mut buf = String::new();

    // -------- Parsing --------
    for d in data.lines() {
        let mut but = Vec::new();
        for k in d.chars() {
            match k {
                '[' => buf.clear(),
                ']' => conf.push(buf.clone()),
                '(' => buf.clear(),
                ')' => but.push(buf.clone()),
                '{' => break,
                _ => buf.push(k),
            }
        }
        buttons.push(but);
    }

    let config: Vec<Vec<bool>> = conf
        .iter()
        .map(|i| i.chars().map(|x| x == '#').collect())
        .collect();

    let button_config: Vec<Vec<Vec<usize>>> = buttons
        .iter()
        .map(|x| {
            x.iter()
                .map(|b| b.split(',').map(|v| v.parse::<usize>().unwrap()).collect())
                .collect()
        })
        .collect();

    let machines: Vec<(&Vec<bool>, Vec<Vec<usize>>)> = config.iter().zip(button_config).collect();

    let mut total_smallest = 0;

    for (target, btns) in machines {
        let n_lights = target.len();
        let mut smallest = 0;

        'outer: for k in 0..=btns.len() {
            for subset in btns.iter().combinations(k) {
                let mut state = vec![false; n_lights];

                for button in subset {
                    for &idx in button.iter() {
                        state[idx] = !state[idx];
                    }
                }

                if state == *target {
                    smallest = k;
                    break 'outer;
                }
            }
        }

        total_smallest += smallest;
    }

    println!("total {:?}", total_smallest);
}
