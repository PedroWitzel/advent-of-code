use std::io::{BufRead, BufReader};

use crate::helpers;

pub fn run() {
    println!("{}\n", part1());
    println!("{}\n", part2());
}

fn part1() -> String {
    let lines = BufReader::new(helpers::input_file(file!())).lines();
    let mut answer = 0;

    lines
        .map(|s| {
            let s = s.unwrap();
            let (side, turn) = s.split_at(1);
            let mut turn = turn.parse::<i32>().unwrap();
            if side == "L" {
                turn = -turn
            }
            turn
        })
        .fold(50, |acc, e| {
            let acc = acc + e;
            if acc % 100 == 0 {
                answer += 1
            }
            acc
        });

    answer.to_string()
}

fn part2() -> String {
    let lines = BufReader::new(helpers::input_file(file!())).lines();
    let mut answer = 0;

    lines
        .map(|s| {
            let s = s.unwrap();
            let (side, turn) = s.split_at(1);
            let mut turn = turn.parse::<i32>().unwrap();
            if side == "L" {
                turn = -turn
            }
            turn
        })
        .fold(1000_50, |acc, e| {
            // Full turns taken into account here
            answer += (e / 100).abs();

            let next = acc + e % 100;

            // Spot on zero
            if next % 100 == 0 {
                answer += 1;
            } else if acc % 100 != 0 && (acc / 100 - next / 100).abs() != 0 {
                // Crossed zero
                answer += 1;
            }

            next
        });

    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "992");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "6133");
    }
}
