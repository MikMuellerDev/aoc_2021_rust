pub fn run() {
    println!("<--- DAY 06 --->");
    let raw_input = include_str!("resources/day6.txt");

    let input: Vec<u8> = raw_input
        .split(',')
        .map(|a| a.parse::<u8>().unwrap())
        .collect();
    part1(&input);
    part2(&input);
}

pub fn part1(input: &Vec<u8>) {
    let mut fish_tank = input.clone();

    for _ in 0..80 {
        for index in 0..fish_tank.len() {
            if fish_tank[index] == 0 {
                fish_tank[index] = 7;
                fish_tank.push(8);
            }

            fish_tank[index] -= 1;
        }
    }
    println!("Part 1: {}", fish_tank.len())
}

pub fn part2(input: &Vec<u8>) {
    let mut fish_lifecycle: Vec<usize> = vec![0; 9];

    // Add initial values of the fishe's age
    for index in 0..input.len() {
        fish_lifecycle[input[index] as usize] += 1;
    }

    // Actual simulation
    for _ in 0..256 {
        let first = fish_lifecycle.remove(0);
        fish_lifecycle[6] += first;
        fish_lifecycle.push(first);
    }

    println!("Part 2: {:?}", fish_lifecycle.iter().sum::<usize>());
}
