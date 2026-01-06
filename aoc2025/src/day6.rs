use std::fs::read_to_string;

use itertools::{Itertools, Position};
use nom::{
    AsChar, IResult, Input, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, space0, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
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
fn operations(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(space1, alt((tag("*"), tag("+")))).parse(input)
}

pub fn day6() {
    let mut ops = vec![];
    let mut lines_iterators = vec![];

    let input = read_to_string("puzzles/puz6.txt").expect("needed");
    for (pos, line) in input.lines().with_position() {
        match pos {
            Position::Last => {
                let (_input, mut output) = operations(line).unwrap();
                output.reverse(); // right → left
                ops = output;
            }
            _ => {
                // ✅ ONLY number lines
                lines_iterators.push(line.chars().rev());
            }
        }
    }

    let result = ops
        .iter()
        .map(|op| {
            let mut output = match *op {
                "*" => 1,
                "+" => 0,
                _ => unreachable!(),
            };

            loop {
                let mut used_any = false;

                let value: u64 = lines_iterators
                    .iter_mut()
                    .rev() // top → bottom
                    .filter_map(|line| {
                        line.next().and_then(|c| c.to_digit(10)).map(|d| {
                            used_any = true;
                            d as u64
                        })
                    })
                    .enumerate()
                    .map(|(place, digit)| digit * 10u64.pow(place as u32))
                    .sum();

                // ✅ correct termination condition
                if !used_any {
                    break;
                }

                match *op {
                    "*" => output *= value,
                    "+" => output += value,
                    _ => unreachable!(),
                }
            }

            output
        })
        .sum::<u64>();

    println!("{result}");
}

// fn parse(input: &str) -> IResult<&str, (Vec<Vec<u64>>, Vec<&str>)> {
//     separated_pair(
//         separated_list1(
//             line_ending,
//             delimited(space0, separated_list1(space1, complete::u64), space0),
//         ),
//         line_ending,
//         separated_list1(space1, alt((tag("*"), tag("+")))),
//     )
//     .parse(input)
// }
