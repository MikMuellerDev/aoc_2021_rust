use std::env::split_paths;

pub fn run() {
    println!("<--- DAY 05 --->");
    let raw_input = include_str!("resources/day5.txt");

    let input:Vec<(&str, &str)> = raw_input
    .split('\n')
    .map(|line|(line.split("->").nth(0).unwrap(), line.split("->").nth(1).unwrap()))
    .collect();

    for line in input {
        println!("{:?}", line)
    }
}
