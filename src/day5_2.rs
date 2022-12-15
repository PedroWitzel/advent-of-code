use std::fs::File;
use std::io::{self, BufRead};

pub fn answer() {
    let file = File::open("resources/day05-input.txt").expect("input file exists");
    let lines = io::BufReader::new(file).lines();

    let mut crates = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];

    let mut loading_crates = true;
    for line in lines {
        if let Ok(line) = line {
            if loading_crates {
                let mut chars = line.chars();

                let item_crate = chars.nth(1).unwrap_or(' ');

                // End of crates
                if item_crate == '1' {
                    loading_crates = false;
                } else {
                    if item_crate != ' ' {
                        crates[0].insert(0, item_crate)
                    }
                    let mut crate_index = 1;
                    loop {
                        match chars.nth(3) {
                            Some(item_crate) => {
                                if item_crate != ' ' {
                                    crates[crate_index].insert(0, item_crate)
                                }
                            }
                            None => break,
                        }
                        crate_index += 1;
                    }
                }
            } else if !line.is_empty() {
                let mut commands = line.split(' ');
                let moves = commands.nth(1).unwrap_or("0").parse::<usize>().unwrap_or(0);
                let origin_crate = commands.nth(1).unwrap_or("0").parse::<usize>().unwrap_or(0) - 1;
                let destination_crate =
                    commands.nth(1).unwrap_or("0").parse::<usize>().unwrap_or(0) - 1;

                let origin_crate = &mut crates[origin_crate];
                let moves = origin_crate.len() - moves;
                let mut crane_has: Vec<char> = origin_crate.drain(moves..).collect();
                crates[destination_crate].append(&mut crane_has);
            }
        }
    }

    for mut c in crates {
        print!("{}", c.pop().unwrap());
    }
    println!("")
}
