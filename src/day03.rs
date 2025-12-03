use std::io::{BufRead, BufReader};

use crate::helpers;

pub fn run() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> String {
    let lines = BufReader::new(helpers::input_file(file!())).lines();
    let answer = lines
        .map(|s| s.unwrap())
        .map(|s| {
            let zero = 0 as char;
            let first = s
                .char_indices()
                .max_by(|a, b| {
                    if b.0 == s.len() - 1 {
                        a.1.cmp(&zero)
                    } else {
                        a.1.cmp(&b.1)
                    }
                })
                .unwrap();
            let first = s.char_indices().find(|a| a.1 == first.1).unwrap();

            let second = s
                .char_indices()
                .skip(first.0 + 1)
                .max_by(|a, b| a.1.cmp(&b.1))
                .unwrap();

            let numb_s = format!("{}{}", first.1, second.1);
            dbg!(&s, &numb_s);
            numb_s.parse::<i32>().unwrap_or_default()
        })
        .sum::<i32>();

    answer.to_string()
}

fn max_in_range(s: &str) -> (usize, char) {
    let mut max = (0 as usize, 0 as char);
    s.char_indices().for_each(|a| {
        if a.1 > max.1 {
            max = a;
        }
    });
    max
}

fn part2() -> String {
    let lines = BufReader::new(helpers::input_file(file!())).lines();
    let answer = lines
        .map(|s| s.unwrap())
        .map(|s| {
            let mut start = 0;
            (0..12)
                .map(|i| {
                    let (index, c) = max_in_range(&s[start..s.len() - 11 + i]);
                    start += index + 1;
                    c
                })
                .map(|c| c.to_string().parse::<i64>().unwrap_or_default())
                .reduce(|acc, value| acc * 10 + value)
                .unwrap()
        })
        .sum::<i64>();
    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "16361");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "169512729575727");
    }
}
