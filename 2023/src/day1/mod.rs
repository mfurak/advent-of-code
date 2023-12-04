use std::fs::File;
use std::io::Read;

pub fn answer() {
    //read sample
    //read input
    let lines = read_into_lines("src/day1/part1/input.txt");
    // println!("{}", lines.join(","));
    println!("{}", solve(&lines));
}

fn solve(lines: &Vec<String>) -> u32 {
    lines
        .into_iter()
        .map(|line| {
            let first = line.chars().find(|c| c.is_digit(10)).unwrap();
            let last = line.chars().rev().find(|c| c.is_digit(10)).unwrap();
            let num = format!("{}{}", first, last);
            let res = num.parse::<u32>().unwrap();
            res
        })
        .sum()
}

fn read_into_lines(filename: &str) -> Vec<String> {
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
