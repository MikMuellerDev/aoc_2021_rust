extern crate regex;
use regex::Regex;

pub fn run() {
    println!("<--- DAY 02 --->");
    let raw_input = include_str!("resources/day2.txt");
    let re = Regex::new(r"(\w*)\s(\d)").unwrap();

    let input: Vec<(&str, u8)> = raw_input
        .split('\n')
        .map(|instruction| {
            let caps = re.captures(instruction).unwrap();
            (
                caps.get(1).unwrap().as_str(),
                caps.get(2).unwrap().as_str().parse::<u8>().unwrap(),
            )
        })
        .collect::<Vec<(&str, u8)>>();
    part1(&input)
}

pub fn part1(input: &Vec<(&str, u8)>) {
    let mut x_pos: u32 = 0;
    let mut y_pos: u32 = 0;
    for index in 0..input.len() {
        match input[index].0 {
            "forward" => {
                x_pos += input[index].1 as u32;
            }
            "down" => {
                y_pos += input[index].1 as u32;
            }
            "up" => y_pos -= input[index].1 as u32,
            _ => {
                println!("Error, invalid instruction!")
            }
        }
    }
    println!("X_POS: {} Y_POS: {}", x_pos, y_pos);
    println!("Puzzle_Solution: {}", x_pos * y_pos)
}
