use std::io::{BufRead, BufReader};

use crate::helpers;

pub fn run() {
    part1();
    part2();
}

fn part1() -> String {
    let lines = BufReader::new(helpers::input_file(file!())).lines();
    let mut answer = 0;

    answer.to_string()
}

fn part2() -> String {
    let lines = BufReader::new(helpers::input_file(file!())).lines();
    let mut answer = 0;

    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part1(), "0");
    }
}
