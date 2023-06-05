use std::collections::HashMap;
use std::fs::File as Std_File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Directory {
    path: String,
    name: String,
    size: i32,
}

fn get_parent_name(path: &str) -> Option<String> {
    let path_split: Vec<_> = path.split("/").collect();
    let dirs = path_split.len();
    match dirs {
        0 | 1 => None,
        _ => Some(path_split[..(dirs - 1)].join("/")),
    }
}

fn add_size_to_parents(db: &mut HashMap<String, Directory>, name: &str, size: i32) {
    if let Some(dir) = db.get_mut(name) {
        dir.size += size;
        if let Some(parent) = get_parent_name(name) {
            add_size_to_parents(db, &parent, size);
        }
    }
}

pub fn answer() {
    let file = Std_File::open("resources/day07-input.txt").expect("input file exists");
    let lines = io::BufReader::new(file).lines();

    let mut current_dir = "".to_string();

    // List of all directories
    let mut directories = HashMap::new();
    directories.insert(
        current_dir.clone(),
        Directory {
            path: "".to_string(),
            name: "".to_string(),
            size: 0,
        },
    );

    for line in lines {
        if let Ok(line) = line {
            // dbg!(&line);
            let mut commands = line.split(' ');
            let first_word = commands.next().expect("Always one");

            if first_word == "$" {
                // read commands
                let command = commands.next().expect("Always one");

                match command {
                    "cd" => {
                        let cd_param = commands.next().expect("Always one dir").to_string();
                        if cd_param == "/" {
                            // ignore :)
                        } else if cd_param == ".." {
                            // Need to get the other directory to
                            current_dir =
                                get_parent_name(&current_dir).expect("Was on a parent already");
                        } else {
                            current_dir = current_dir.clone() + "/" + cd_param.as_ref();
                            directories.insert(
                                current_dir.clone(),
                                Directory {
                                    path: current_dir.clone(),
                                    name: cd_param,
                                    size: 0,
                                },
                            );
                        }
                    }
                    "ls" => (),
                    _ => (),
                }
            } else {
                // reading output
                match first_word {
                    "dir" => {}
                    _ => {
                        let size = first_word.parse::<i32>().expect("file size present");
                        add_size_to_parents(&mut directories, &current_dir, size);
                    }
                }
            }
        }
    }

    // Find directories with at most 100000 of size
    let answer: i32 = directories
        .values()
        .filter(|&dir| dir.size < 100000)
        .map(|dir| dir.size)
        .sum();
    dbg!(&answer);
}
