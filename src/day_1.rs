pub fn run() {
    println!("<--- DAY 01 --->");
    let raw_input = include_str!("resources/day1.txt");
    let input: Vec<&str> = raw_input.split("\n").collect();
    part1(&input);
    part2(&input);
}

pub fn part1(input: &Vec<&str>) {
    let mut increases: u32 = 0;

    for index in 0..input.len() {
        if index != 0 && index - 1 < input.len() {
            let previous: u32 = input[index - 1].parse::<u32>().unwrap();
            if input[index].parse::<u32>().unwrap() > previous {
                increases += 1;
            }
        }
    }

    println!("Part 1: {}", increases);
}

pub fn part2(input: &Vec<&str>) {
    let mut processed: Vec<u32> = vec![];
    for index in 0..input.len() {
        if index + 1 < input.len() && index + 2 < input.len() {
            let current: u32 = input[index].parse::<u32>().unwrap()
                + input[index + 1].parse::<u32>().unwrap()
                + input[index + 2].parse::<u32>().unwrap();
            processed.push(current);
        }
    }
    let mut increases: u32 = 0;

    for index in 0..processed.len() {
        if index != 0 && index - 1 < processed.len() {
            let previous: u32 = processed[index - 1];
            if processed[index] > previous {
                increases += 1;
            }
        }
    }
    println!("Part 2: {}", increases);
}
