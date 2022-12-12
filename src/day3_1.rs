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
    let mut bad_items = Vec::new();

    for line in lines {
        if let Ok(items) = line {
            let items = items.as_bytes();
            let compartment_size = items.len() / 2;

            // Slice the compartment into two arrays
            let first_compartment = &items[..compartment_size];
            let second_compartment = &items[compartment_size..];

            for item in first_compartment {
                if second_compartment.iter().any(|x| x == item) {
                    bad_items.push(*item);
                    break;
                }
            }
        }
    }

    let points: Vec<_> = bad_items.iter().map(|x| to_point(x)).collect();
    let points: i32 = points.iter().sum();
    println!("{:?}", points);
}
