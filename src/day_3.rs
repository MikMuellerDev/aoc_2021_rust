pub fn run() {
    println!("<--- DAY 03 --->");
    let raw_input = include_str!("resources/day3.txt");

    let input: Vec<&str> = raw_input.split('\n').collect();

    part1(&input);
    get_oxygen(&input, 0);
    get_co2(&input, 0)
}

pub fn part1(input: &Vec<&str>) {
    let char_len: u8 = input[0].chars().count() as u8;

    let mut output_gamma: Vec<char> = vec![];
    let mut output_epsilon: Vec<char> = vec![];

    for i in 0..char_len {
        let mut count_0: u32 = 0;
        let mut count_1: u32 = 0;

        for line in input {
            let current_byte = line.chars().nth(i as usize).unwrap();
            if current_byte == '0' {
                count_0 += 1;
            } else {
                count_1 += 1;
            }
        }

        if count_0 > count_1 {
            output_gamma.push('0');
            output_epsilon.push('1');
        } else {
            output_gamma.push('1');
            output_epsilon.push('0');
        }
    }

    let gamma: u32 =
        isize::from_str_radix(&output_gamma.iter().cloned().collect::<String>(), 2).unwrap() as u32;

    let epsilon: u32 = isize::from_str_radix(&output_epsilon.iter().cloned().collect::<String>(), 2)
        .unwrap() as u32;

    println!("Part 1: {}", gamma * &epsilon)
}

pub fn get_oxygen(input: &Vec<&str>, start: u8) {
    let char_len: u8 = input[0].chars().count() as u8;

    let mut current_dataset_oxygen: Vec<&str> = vec![];

    let mut count_0: u32 = 0;
    let mut count_1: u32 = 0;

    for line in input {
        let current_byte = line.chars().nth(start as usize).unwrap();
        if current_byte == '0' {
            count_0 += 1;
        } else {
            count_1 += 1;
        }
    }

    if count_0 > count_1 {
        for line in input {
            if line.chars().nth(start as usize).unwrap() == '0' {
                current_dataset_oxygen.push(line)
            }
        }
    } else if count_1 > count_0 {
        for line in input {
            if line.chars().nth(start as usize).unwrap() == '1' {
                current_dataset_oxygen.push(line)
            }
        }
    } else if count_0 == count_1 {
        for line in input {
            if line.chars().nth(start as usize).unwrap() == '1' {
                current_dataset_oxygen.push(line)
            }
        }
    }

    if current_dataset_oxygen.len() > 1 {
        get_oxygen(&current_dataset_oxygen, start + 1);
    } else {
        let result: u32 = isize::from_str_radix(&current_dataset_oxygen[0], 2).unwrap() as u32;
        println!("Part 2 (Oxygen): {}", result);
    }
}

pub fn get_co2(input: &Vec<&str>, start: u8) {
    let char_len: u8 = input[0].chars().count() as u8;

    let mut current_dataset_co2: Vec<&str> = vec![];

    let mut count_0: u32 = 0;
    let mut count_1: u32 = 0;

    for line in input {
        let current_byte = line.chars().nth(start as usize).unwrap();
        if current_byte == '0' {
            count_0 += 1;
        } else {
            count_1 += 1;
        }
    }

    if count_0 > count_1 {
        for line in input {
            if line.chars().nth(start as usize).unwrap() == '1' {
                current_dataset_co2.push(line)
            }
        }
    } else if count_1 > count_0 {
        for line in input {
            if line.chars().nth(start as usize).unwrap() == '0' {
                current_dataset_co2.push(line)
            }
        }
    } else if count_0 == count_1 {
        for line in input {
            if line.chars().nth(start as usize).unwrap() == '0' {
                current_dataset_co2.push(line)
            }
        }
    }

    if current_dataset_co2.len() > 1 {
        get_co2(&current_dataset_co2, start + 1);
    } else {
        let result: u32 = isize::from_str_radix(&current_dataset_co2[0], 2).unwrap() as u32;
        println!("Part 2 (Co2): {}", result);
    }
}
