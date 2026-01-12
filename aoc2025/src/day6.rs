use std::fs::read_to_string;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space0, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
};
// pub fn day6() {
//     let input = read_to_string("puzzles/puz6.txt").expect("needed");
//     let (_input, (nums, ops)) = parse.parse(&input).unwrap();
//     let result = ops
//         .iter()
//         .enumerate()
//         .map(|(index, op)| {
//             let it: Vec<u64> = (0..nums.len())
//                 .into_iter()
//                 .map(|iner_index| nums[iner_index][index])
//                 .collect();
//             match *op {
//                 "*" => it.iter().product(),
//                 "+" => it.iter().sum::<u64>(),
//                 _ => {
//                     panic!("")
//                 }
//             }
//         })
//         .sum::<u64>();
//     println!("{:?}", result);
// }

pub fn day6() {
    let input = read_to_string("puzzles/puz6.txt").expect("needed");
    let (_input, (nums, ops)) = parse.parse(&input).unwrap();

    let total: u64 = ops
        .iter()
        .enumerate()
        .map(|(index, op)| {
            // take column numbers vertically (one "problem")
            let it: Vec<u64> = (0..nums.len()).map(|row| nums[row][index]).collect();

            // digits per row, reversed (LSB first)
            let d: Vec<Vec<char>> = it
                .iter()
                .map(|x| x.to_string().chars().rev().collect())
                .collect();

            let max = d.iter().map(|v| v.len()).max().unwrap_or(0);

            // build operands by digit-column (right-to-left)
            let operands: Vec<u64> = (0..max)
                .map(|x| {
                    let mut value: u64 = 0;
                    for y in 0..d.len() {
                        if let Some(ch) = d[y].get(x) {
                            let digit = (*ch as u8 - b'0') as u64;
                            value = value * 10 + digit;
                        }
                    }
                    value
                })
                .collect();

            match *op {
                "*" => operands.into_iter().product::<u64>(),
                "+" => operands.into_iter().sum::<u64>(),
                _ => unreachable!(),
            }
        })
        .sum();

    println!("{total}");
}

fn parse(input: &str) -> IResult<&str, (Vec<Vec<u64>>, Vec<&str>)> {
    separated_pair(
        separated_list1(
            line_ending,
            delimited(space0, separated_list1(space1, complete::u64), space0),
        ),
        line_ending,
        separated_list1(space1, alt((tag("*"), tag("+")))),
    )
    .parse(input)
}
