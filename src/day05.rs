use std::io::{BufRead, BufReader};

use crate::helpers;

pub fn run() {
    println!("{}", part1());
    println!("{}", part2());
}

fn part1() -> String {
    let mut lines = BufReader::new(helpers::input_file(file!())).lines();

    let ranges = lines
        .by_ref()
        .take_while(|s| s.as_ref().is_ok_and(|l| !l.is_empty()))
        .map(|s| s.unwrap())
        .map(|s| {
            let (i, j) = s.split_once('-').unwrap();
            (i.parse::<u64>().unwrap())..=(j.parse::<u64>().unwrap())
        })
        .collect::<Vec<_>>();
    // TODO - consolidate ranges

    // Skip empty lines
    lines
        .map(|s| s.unwrap())
        .map(|s| s.parse::<u64>().unwrap())
        .filter(|s| ranges.iter().any(|range| range.contains(s)))
        .count()
        .to_string()
}

fn part2() -> String {
    let lines = BufReader::new(helpers::input_file(file!())).lines();

    let mut ranges = lines
        .take_while(|s| s.as_ref().is_ok_and(|l| !l.is_empty()))
        .flat_map(|s| s)
        .map(|s| {
            if let Some((i, j)) = s.split_once('-') {
                Some((i.parse::<u64>().unwrap(), j.parse::<u64>().unwrap()))
            } else {
                None
            }
        })
        .flat_map(|s| s)
        .collect::<Vec<_>>();

    ranges.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let mut ranges = ranges.into_iter();
    let mut next_range_maybe = ranges.next();
    let mut current_range = next_range_maybe.unwrap_or_default();

    let mut final_ranges = vec![];
    next_range_maybe = ranges.next();
    while let Some(next_range) = next_range_maybe {
        let range = current_range.0..=current_range.1;

        if range.contains(&next_range.0) {
            if next_range.1 > current_range.1 {
                current_range.1 = next_range.1;
            }
        } else {
            final_ranges.push(current_range);
            current_range = next_range;
        }
        next_range_maybe = ranges.next();
    }
    final_ranges.push(current_range);

    final_ranges
        .into_iter()
        .map(|x| x.1 - x.0 + 1)
        .sum::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "848");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "334714395325710");
    }
}
