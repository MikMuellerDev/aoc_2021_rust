pub fn run() {
    println!("<--- DAY 09 --->");
    let raw_input = include_str!("resources/day9.txt");
    let input: Vec<Vec<u8>> = raw_input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|num| num.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    part1(&input);
}

pub fn part1(input: &Vec<Vec<u8>>) {
    let mut low_points: Vec<u16> = vec![];
    for (line_index, line) in input.iter().enumerate() {
        for (num_index, point) in line.iter().enumerate() {
            let above: u8;
            let below: u8;
            let left: u8;
            let right: u8;

            if line_index == 0 {
                above = point.clone() + 1;
            } else {
                above = input[line_index - 1][num_index]
            }

            if line_index == input.len() - 1 {
                below = point.clone() + 1;
            } else {
                below = input[line_index + 1][num_index]
            }

            if num_index == 0 {
                left = point.clone() + 1;
            } else {
                left = line[num_index - 1]
            }

            if num_index == line.len() - 1 {
                right = point.clone() + 1;
            } else {
                right = line[num_index + 1]
            }

            if point < &above && point < &below && point < &left && point < &right {
                low_points.push(*point as u16)
            }
        }
    }

    // println!("Low Points:    {:?}", low_points);
    println!("Part 1: {:?}", low_points.iter().map(|value|value + 1).collect::<Vec<u16>>().iter().sum::<u16>());
}