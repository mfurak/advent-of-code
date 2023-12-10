use crate::reader::read_into_lines;

pub fn answer() {
    //read sample
    //read input
    let part1_lines = read_into_lines("src/day1/part1/input.txt");
    println!("Part 1: {}", part1(&part1_lines));
    let part2_lines = read_into_lines("src/day1/part2/input.txt");
    println!("Part 2: {}", part2(&part2_lines));
}

fn part1(lines: &Vec<String>) -> u32 {
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

fn part2(lines: &Vec<String>) -> u32 {
    0
}
