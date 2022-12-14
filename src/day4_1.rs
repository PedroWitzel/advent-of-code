use std::fs::File;
use std::io::{self, BufRead};

pub fn answer() {
    let file = File::open("resources/sample.txt").expect("input file exists");
    let lines = io::BufReader::new(file).lines();

    let mut overlaps = 0;

    for line in lines {
        if let Ok(assignments) = line {
            let mut groups = assignments.split(|x| x == '-' || x == ',');

            let elf1_start = groups
                .next()
                .expect("There's at least one item")
                .parse::<i32>()
                .expect("a num");
            let elf1_end = groups
                .next()
                .expect("There's at least two item")
                .parse::<i32>()
                .expect("a num");
            let elf2_start = groups
                .next()
                .expect("There's at least one item")
                .parse::<i32>()
                .expect("a num");
            let elf2_end = groups
                .next()
                .expect("There's at least two item")
                .parse::<i32>()
                .expect("a num");

            if (elf1_start <= elf2_start && elf1_end >= elf2_end)
                || (elf2_start <= elf1_start && elf2_end >= elf1_end)
            {
                overlaps += 1;
            }
        }
    }

    println!("{:?}", overlaps);
}
