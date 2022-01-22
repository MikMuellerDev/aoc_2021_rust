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
    println!("Calculating trajectory intersections for Part 1...");
    part1(&input);
    println!("Calculating trajectory intersections for Part 2...");
    part2(&input);
}

pub fn part1(input: &Vec<(u32, u32, u32, u32)>) {
    let mut existing_coordinates: Vec<(u32, u32)> = vec![];
    let mut marked_coords: Vec<(u32, u32)> = vec![];
    let mut intersection_count: u32 = 0;

    for trajec in input {
        let mut x_1: u32 = trajec.0;
        let mut x_2: u32 = trajec.2;
        let mut y_1: u32 = trajec.1;
        let mut y_2: u32 = trajec.3;

        if x_1 == x_2 {
            if y_1 > y_2 {
                let temp = y_1;
                y_1 = y_2;
                y_2 = temp;
            }

            for p_y in y_1..y_2 + 1 {
                if !existing_coordinates.contains(&(x_1, p_y)) {
                    existing_coordinates.push((x_1, p_y));
                } else if !marked_coords.contains(&(x_1, p_y)) {
                    intersection_count += 1;
                    marked_coords.push((x_1, p_y))
                }
            }
        } else if y_1 == y_2 {
            if x_1 > x_2 {
                let temp = x_1;
                x_1 = x_2;
                x_2 = temp;
            }

            for p_x in x_1..x_2 + 1 {
                if !existing_coordinates.contains(&(p_x, y_1)) {
                    existing_coordinates.push((p_x, y_1));
                } else if !marked_coords.contains(&(p_x, y_1)) {
                    intersection_count += 1;
                    marked_coords.push((p_x, y_1))
                }
            }
        } else {
        }
    }
    println!("Part 1: {}", intersection_count);
}

pub fn part2(input: &Vec<(u32, u32, u32, u32)>) {
    let mut existing_coordinates: Vec<(u32, u32)> = vec![];
    let mut marked_coords: Vec<(u32, u32)> = vec![];
    let mut intersection_count: u32 = 0;

    for trajec in input {
        let mut x_1: u32 = trajec.0;
        let mut y_1: u32 = trajec.1;
        let mut x_2: u32 = trajec.2;
        let mut y_2: u32 = trajec.3;

        if x_1 == x_2 {
            if y_1 > y_2 {
                let temp = y_1;
                y_1 = y_2;
                y_2 = temp;
            }

            for p_y in y_1..y_2 + 1 {
                if !existing_coordinates.contains(&(x_1, p_y)) {
                    existing_coordinates.push((x_1, p_y));
                } else if !marked_coords.contains(&(x_1, p_y)) {
                    intersection_count += 1;
                    marked_coords.push((x_1, p_y))
                }
            }
        } else if y_1 == y_2 {
            if x_1 > x_2 {
                let temp = x_1;
                x_1 = x_2;
                x_2 = temp;
            }

            for p_x in x_1..x_2 + 1 {
                if !existing_coordinates.contains(&(p_x, y_1)) {
                    existing_coordinates.push((p_x, y_1));
                } else if !marked_coords.contains(&(p_x, y_1)) {
                    intersection_count += 1;
                    marked_coords.push((p_x, y_1))
                }
            }
        } else {    
            let mut p_x = x_1;
            let mut p_y = y_1;
            
            loop {        
                if !existing_coordinates.contains(&(p_x, p_y)) {
                    existing_coordinates.push((p_x, p_y));
                } else if !marked_coords.contains(&(p_x, p_y)) {
                    intersection_count += 1;
                    marked_coords.push((p_x, p_y))
                }

                if p_x == x_2 {
                    break;
                }



                if x_1 < x_2 {
                    p_x += 1;
                } else {
                    p_x -= 1;
                }

                if y_1 < y_2 {
                    p_y += 1;
                } else {
                    p_y -= 1;
                }

            }
        }
    }
    println!("Part 2: {}", intersection_count);
}
