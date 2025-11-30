use std::fs::File;
use std::path::Path;

pub fn input_file(file: &str) -> File {
    let day = Path::new(file)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap();
    let input_file = format!("resources/{day}-input.txt");
    File::open(&input_file).unwrap()
}
