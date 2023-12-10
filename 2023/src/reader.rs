use std::fs::File;
use std::io::Read;

pub fn read_into_lines(filename: &str) -> Vec<String> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("404 - File not found :("),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("Unable to read file");
    let lines: Vec<String> = file_contents.lines().map(|s| s.to_owned()).collect();
    lines
}
