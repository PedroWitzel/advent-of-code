use std::fs::File;
use std::io::{self, BufRead};

fn get_number(value: Option<&str>) -> i32 {
    let mut num = 0;

    match value {
        Some(num_string) => match num_string.parse::<i32>() {
            Ok(num_i32) => num = num_i32,
            Err(_) => (),
        },
        None => (),
    }
    num
}

pub fn answer() {
    let file = File::open("resources/sample.txt").expect("input file exists");
    let lines = io::BufReader::new(file).lines();

    let mut overlaps = 0;

    for line in lines {
        if let Ok(assignments) = line {
            let mut groups = assignments.split(|x| x == '-' || x == ',');

            let elf1_start = get_number(groups.next());
            let elf1_end = get_number(groups.next());
            let elf2_start = get_number(groups.next());
            let elf2_end = get_number(groups.next());

            if (elf2_start >= elf1_start && elf2_start <= elf1_end)
                || (elf2_end >= elf1_start && elf2_end <= elf1_end)
                || (elf1_start >= elf2_start && elf1_start <= elf2_end)
                || (elf1_end >= elf2_start && elf1_end <= elf2_end)
            {
                overlaps += 1;
            }
        }
    }

    println!("{:?}", overlaps);
}
