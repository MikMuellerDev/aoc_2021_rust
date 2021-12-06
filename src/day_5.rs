use std::env::split_paths;

pub fn run() {
    println!("<--- DAY 05 --->");
    let raw_input = include_str!("resources/day5.txt");

    let input: Vec<(u32, u32, u32, u32)> = raw_input
        .split('\n')
        .map(|line| {
            (
                line.split("->")
                    .nth(0)
                    .unwrap()
                    .split(',')
                    .nth(0)
                    .unwrap()
                    .replace(' ', "")
                    .parse::<u32>()
                    .unwrap(),
                line.split("->")
                    .nth(0)
                    .unwrap()
                    .split(',')
                    .nth(1)
                    .unwrap()
                    .replace(' ', "")
                    .parse::<u32>()
                    .unwrap(),
                line.split("->")
                    .nth(1)
                    .unwrap()
                    .split(',')
                    .nth(0)
                    .unwrap()
                    .replace(' ', "")
                    .parse::<u32>()
                    .unwrap(),
                line.split("->")
                    .nth(1)
                    .unwrap()
                    .split(',')
                    .nth(1)
                    .unwrap()
                    .replace(' ', "")
                    .parse::<u32>()
                    .unwrap(),
            )
        })
        .collect();
    part1(&input)
}

pub fn part1(input: &Vec<(u32, u32, u32, u32)>) {
    for line in input {
        let mut x_1: u32 = line.0;
        let mut x_2: u32 = line.2;
        let mut y_1: u32 = line.1;
        let mut y_2: u32 = line.3;

        println!("x1: {}, x2: {}, x3: {}, x4: {}", x_1, x_2, y_1, y_2);
    }
}
