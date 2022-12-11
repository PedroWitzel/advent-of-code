use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq)]
enum Jokenpo {
    Rock,
    Paper,
    Scissors
}

impl Jokenpo {

    pub fn from(c: char) -> Result<Jokenpo, ()> {
        match c {
            'A' | 'X' => Ok(Jokenpo::Rock),
            'B' | 'Y' => Ok(Jokenpo::Paper),
            'C' | 'Z' => Ok(Jokenpo::Scissors),
            _ => Err(()),
        }
    }

    pub fn points(me: Jokenpo, them: Jokenpo) -> i32 {
        let mut points = match me {
            Jokenpo::Rock => 1,
            Jokenpo::Paper => 2,
            Jokenpo::Scissors => 3,
        };

        if me == them {
            points += 3;
        } else if (me == Jokenpo::Rock && them == Jokenpo::Scissors) || 
                (me == Jokenpo::Paper && them == Jokenpo::Rock) ||
                    (me == Jokenpo::Scissors && them == Jokenpo::Paper) {
            points += 6;
        }
        points
    }
}

pub fn answer() {
    
    let file = File::open("resources/day02-input.txt").unwrap();
    //let file = File::open("resources/sample.txt").unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut match_points = 0;

    for line in lines {
        if let Ok(game) = line {
            let mut options = game.split(' ');
            let them = Jokenpo::from(options.next().unwrap().chars().nth(0).unwrap());
            let me = Jokenpo::from(options.next().unwrap().chars().nth(0).unwrap());

            let them = them.unwrap();
            let me = me.unwrap();
            let points = Jokenpo::points(me, them);
            match_points += points;
        }
    }

    println!("{}", match_points);
}

