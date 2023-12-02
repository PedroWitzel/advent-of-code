use std::fs::File;
use std::io::{self, BufRead};


pub fn part1() -> String {
    let file = File::open("resources/day01-input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut coordinates = 0;
    for line in lines {
        if let Ok(calibration) = line {
            let mut numbers = calibration.chars().filter(|c| c.is_numeric());
            let first = numbers.next().unwrap().to_digit(10).unwrap();
            if let Some(last) = numbers.next_back() {
                coordinates += last.to_digit(10).unwrap() + first * 10;
            } else {
                coordinates += first * 11
            }
        }
    }

    coordinates.to_string()
}
