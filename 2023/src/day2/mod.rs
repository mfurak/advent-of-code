use crate::reader::read_into_lines;
use regex:: Regex ;

pub fn solve() {
    let part1_lines = read_into_lines("src/day2/part1/sample.txt");
    println!("Part 1: {}", part1(&part1_lines));
    // let part2_lines = read_into_lines("src/day2/part2/input.txt");
    // println!("Part 2: {}", part2(&part2_lines));
}
struct Turn {
    red:        i32,
    green: i32,
    blue: i32,
}
impl Turn {
    fn is_possible(&self, other_turn: &Turn) -> bool {
        if *&self.red - other_turn.red < 0
            || *&self.green - other_turn.green < 0
            || *&self.blue - other_turn.blue < 0
        {
            true
        } else {
            false
        }
    }
}

fn part1(lines: &Vec<String>) -> u32 {
    // parse the input
    // subtract game def from each game
    //
    // go through result and if any games results <0 then game is not possible

    let game_number = Regex::new(r"Game (\d+)").unwrap();
    let turn = Regex::new(r"(\d+) ([a-zA-Z]+)").unwrap();
    let res = lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();
            // first part is the game number
            game_number.captures_iter(parts[0]);
            // second part is the turns collect with the group iterator
            let turns_raw:Vec<&str> = parts[1].split(";").collect();
            let turn_list:Vec<&str> = turns_raw.split(" ").collect();
        })
        .collect();

    let sum = 0;
    sum
}

fn part2(lines: &Vec<String>) -> u32 {
    0
}
