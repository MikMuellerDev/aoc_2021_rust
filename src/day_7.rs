pub fn run() {
    println!("<--- DAY 07 --->");
    let raw_input = include_str!("resources/day7.txt");

    let input: Vec<u32> = raw_input
        .split(',')
        .map(|character| character.parse::<u32>().unwrap())
        .collect();
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<u32>) {
    let mut all_values: Vec<u32> = vec![];

    for modify_value in input {
        let mut fuel_count = 0;
        for num in input {
            if num > &modify_value {
                fuel_count += num - modify_value;
            } else {
                fuel_count += modify_value - num;
            }
        }
        all_values.push(fuel_count)
    }

    let mut min_value = all_values[0];
    for value in all_values {
        if value < min_value {
            min_value = value;
        }
    }
    println!("Minimum Fuel Part 1: {}", min_value)
}

fn part2(input: &Vec<u32>) {
    let mut all_values: Vec<u32> = vec![];

    for modify_value in *input.iter().min().unwrap()..=*input.iter().max().unwrap() {
    let mut fuel_count: u32 = 0;
    for num in input {
        if num > &modify_value {
            let mut fuel_increase: u32 = 1;
            for _ in modify_value.clone()..num.clone() {
                fuel_count += fuel_increase;
                fuel_increase += 1;
            }
        } else {
            let mut fuel_increase: u32 = 1;
            for _ in num.clone()..modify_value.clone() {
                fuel_count += fuel_increase;
                fuel_increase += 1;
            }
        }
    }
    all_values.push(fuel_count);
    }

    let mut min_value = all_values[0];
    for value in all_values {
        if value < min_value {
            min_value = value;
        }
    }
    println!("Minimum Fuel Part 2: {}", min_value)
}