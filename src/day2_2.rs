use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq)]
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

    pub fn choice(&self, expected_result: char) -> Jokenpo {
        return match expected_result {
            'Y' => self.clone(),
            'X' => {
                // Loose
                match self {
                    Jokenpo::Rock => Jokenpo::Scissors,
                    Jokenpo::Paper => Jokenpo::Rock,
                    Jokenpo::Scissors => Jokenpo::Paper,
                }
            },
            'Z' => {
                // Win
                match self {
                    Jokenpo::Rock => Jokenpo::Paper,
                    Jokenpo::Paper => Jokenpo::Scissors,
                    Jokenpo::Scissors => Jokenpo::Rock,
                }
            },
            _ => self.clone(),
        }
    }

    pub fn value(&self) -> i32 {
        match self {
            Jokenpo::Rock => return 1,
            Jokenpo::Paper => return 2,
            Jokenpo::Scissors => return 3,
        };
    }

    pub fn points(me: Jokenpo, them: Jokenpo) -> i32 {
        let mut points = me.value();
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
    let lines = io::BufReader::new(file).lines();
    let mut match_points = 0;

    for line in lines {
        if let Ok(game) = line {
            let mut options = game.split(' ');
            // Get line inputs
            let them = Jokenpo::from(options.next().unwrap().chars().nth(0).unwrap());
            let them = them.unwrap();
            let final_result = options.next().unwrap().chars().nth(0).unwrap();

            let me = them.choice(final_result);
            let points = Jokenpo::points(me, them);
            match_points += points;
        }
    }

    println!("{}", match_points);
}

