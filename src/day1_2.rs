use std::fs::File;
use std::io::{self, BufRead};

pub fn answer() {
    let file = File::open("resources/day01-input.txt").unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut elfs_pack = Vec::new();
    let mut elf_calories = 0;

    for line in lines {
        if let Ok(calories) = line {
            match calories.parse::<i32>() {
                Ok(calories) => {
                    elf_calories += calories;
                }
                Err(_) => {
                    elfs_pack.push(elf_calories);
                    elf_calories = 0
                }
            }
        }
    }

    if elf_calories > 0 {
        elfs_pack.push(elf_calories);
    }

    elfs_pack.sort_unstable_by(|a, b| b.cmp(a));

    let top3_max = elfs_pack[0] + elfs_pack[1] + elfs_pack[2];

    println!("Top3 sum is: {}", top3_max);
}
