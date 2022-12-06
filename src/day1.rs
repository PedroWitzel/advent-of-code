use std::fs::File;
use std::io::{self, BufRead};

pub fn answer() {
    
    let file = File::open("resources/sample.txt").unwrap();
    
    let lines = io::BufReader::new(file).lines();

    for line in lines {
        if let Ok(calories) = line {
            println!("{}", calories);
        }
    }

}


