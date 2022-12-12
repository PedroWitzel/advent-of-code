use std::fs::File;
use std::io::{self, BufRead};

fn to_point(item: &u8) -> i32 {
    const a: u8 = 'a' as u8;
    const A: u8 = 'A' as u8;

    if *item >= a {
        return (item - a + 1).into();
    } else {
        return (item - A + 27).into();
    }
}

pub fn answer() {
    let file = File::open("resources/day03-input.txt").expect("input file exists");
    let lines = io::BufReader::new(file).lines();

    // List of items out of order
    let mut badges = Vec::new();
    let mut possible_badges = Vec::new();
    let mut previous_item = "".to_string();

    for line in lines {
        if let Ok(items) = line {
            if previous_item.is_empty() {
                previous_item = items;
            } else if possible_badges.is_empty() {
                for item in items.as_bytes() {
                    if previous_item.as_bytes().iter().any(|x| x == item) {
                        possible_badges.push(*item);
                    }
                }
                previous_item = items;
            } else {
                for item in items.as_bytes() {
                    if possible_badges.iter().any(|x| x == item) {
                        badges.push(item.clone());
                        break;
                    }
                }

                previous_item.clear();
                possible_badges.clear();
            }
        }
    }

    let points: Vec<_> = badges.iter().map(|x| to_point(x)).collect();
    let points: i32 = points.iter().sum();
    println!("{:?}", points);
}
