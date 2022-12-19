use std::fs::File;
use std::io::{self, BufRead};
const MAX: usize = 14;
fn is_marker(code: &[u8]) -> bool {
    let mut double_found = false;
    for outter in 0..(MAX - 1) {
        for inner in (outter + 1)..MAX {
            if code[outter] == code[inner] {
                double_found = true;
                break;
            }
        }
        if double_found {
            break;
        }
    }
    return !double_found;
}

pub fn answer() {
    let file = File::open("resources/day06-input.txt").expect("input file exists");
    let lines = io::BufReader::new(file).lines();

    let mut index = 0;
    for line in lines {
        if let Ok(line) = line {
            let chars = line.as_bytes();
            for _ in 0..(chars.len() - (MAX - 1)) {
                if is_marker(&chars[index..(index + MAX)]) {
                    println!("{}", index + MAX);
                    break;
                }
                index += 1;
            }
        }
    }
}
