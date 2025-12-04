use std::io::{BufRead, BufReader};

use crate::helpers;

type Array<T> = Vec<Vec<T>>;
struct Grid<T> {
    pub grid: Array<T>,
    columns: usize,
    rolls: usize,
}

impl<T> Grid<T> {
    fn new(grid: Array<T>) -> Self {
        Grid {
            columns: grid.first().unwrap().len(),
            rolls: grid.len(),
            grid: grid,
        }
    }
}

impl Grid<bool> {
    fn is_paper_rool(&self, i: usize, j: usize) -> bool {
        self.grid[i][j]
    }

    fn can_move(&self, i: usize, j: usize) -> bool {
        let i_start = i.saturating_sub(1);
        let adjecent_rools = (i_start..=(i + 1)).fold(0, |acc, ii| {
            let j_start = j.saturating_sub(1);

            (j_start..=(j + 1)).fold(0, |acc_2, jj| {
                if ii < self.rolls
                    && jj < self.columns
                    && !(i == ii && j == jj)
                    && self.is_paper_rool(ii, jj)
                {
                    acc_2 + 1
                } else {
                    acc_2
                }
            }) + acc
        });
        adjecent_rools < 4
    }

    fn remove_at(&mut self, i: usize, j: usize) {
        self.grid[i][j] = false;
    }
}

pub fn run() {
    println!("{}", part1());
    println!("{}", part2());
}

const ROLL: char = '@';

fn part1() -> String {
    let lines = BufReader::new(helpers::input_file(file!())).lines();

    let grid = lines
        .map(|s| {
            s.unwrap()
                .chars()
                .map(|c| c.eq(&ROLL))
                .collect::<Vec<bool>>()
        })
        .collect::<Array<bool>>();

    let grid = Grid::new(grid);

    let answer = (0..grid.rolls).fold(0, |acc, i| {
        (0..grid.columns)
            .filter(|j| grid.is_paper_rool(i, *j) && grid.can_move(i, *j))
            .count()
            + acc
    });

    answer.to_string()
}

fn part2() -> String {
    let lines = BufReader::new(helpers::input_file(file!())).lines();

    let grid = lines
        .map(|s| {
            s.unwrap()
                .chars()
                .map(|c| c.eq(&ROLL))
                .collect::<Vec<bool>>()
        })
        .collect::<Array<bool>>();

    let mut grid = Grid::new(grid);

    let mut answer = 0;
    let mut remove_paper = true;
    let mut idx_to_remove = Vec::new();

    while remove_paper {
        remove_paper = false;
        (0..grid.rolls).for_each(|i| {
            (0..grid.columns)
                .filter(|j| grid.is_paper_rool(i, *j) && grid.can_move(i, *j))
                .for_each(|j| {
                    idx_to_remove.push((i, j));
                    remove_paper = true;
                });
        });

        answer += idx_to_remove.len();
        for (i, j) in &idx_to_remove {
            grid.remove_at(*i, *j);
        }
        idx_to_remove.clear();
    }

    answer.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "1486");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "9024");
    }
}
