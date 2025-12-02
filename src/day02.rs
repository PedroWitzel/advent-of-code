use std::io::{BufRead, BufReader};

use crate::helpers;

pub fn run() {
    println!("{}", part1());
    println!("{}", part2());
}

fn has_pattern(value: &i64) -> bool {
    let value_s = value.to_string();
    let len = value_s.len();
    if len % 2 == 0 {
        let (begin, end) = value_s.split_at(len / 2);
        end == begin
    } else {
        false
    }
}

fn has_more_patterns(value: &i64) -> bool {
    let value_s = value.to_string();
    let len = value_s.len();
    (1..=(len / 2))
        .find(|max| {
            let (base, _) = value_s.split_at(*max);
            let test_val = base.repeat((len / max));
            test_val == value_s
        })
        .is_some()
}

fn part1() -> String {
    let mut line: String = Default::default();
    let _ = BufReader::new(helpers::input_file(file!())).read_line(&mut line);
    let answer = line
        .split(',')
        .map(|s| {
            let (lower, upper) = s.split_once('-').unwrap();
            (
                lower.trim().parse::<i64>().unwrap(),
                upper.trim().parse::<i64>().unwrap(),
            )
        })
        .map(|(lower_i, upper_i)| (lower_i..=upper_i).filter(|i| has_pattern(i)))
        .flatten()
        .reduce(|acc, i| acc + i)
        .unwrap_or_default();

    answer.to_string()
}

fn part2() -> String {
    let mut line: String = Default::default();
    let _ = BufReader::new(helpers::input_file(file!())).read_line(&mut line);
    let answer = line
        .split(',')
        .map(|s| {
            let (lower, upper) = s.split_once('-').unwrap();
            (
                lower.trim().parse::<i64>().unwrap(),
                upper.trim().parse::<i64>().unwrap(),
            )
        })
        .map(|(lower_i, upper_i)| (lower_i..=upper_i).filter(|i| has_more_patterns(i)))
        .flatten()
        .reduce(|acc, i| acc + i)
        .unwrap_or_default();
    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "54641809925");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "73694270688");
    }
}
