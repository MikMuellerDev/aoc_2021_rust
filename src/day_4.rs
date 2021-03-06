pub fn run() {
    println!("<--- DAY 04 --->");
    let raw_input = include_str!("resources/day4.txt");

    let instructions: Vec<u16> =
        "83,5,71,61,88,55,95,6,0,97,20,16,27,7,79,25,81,29,22,52,43,21,53,59,99,18,35,96,51,93,14,77,15,3,57,28,58,17,50,32,74,63,76,84,65,9,62,67,48,12,8,68,31,19,36,85,98,30,91,89,66,80,75,47,4,23,60,70,87,90,13,38,56,34,46,24,41,92,37,49,73,10,94,26,42,40,33,54,86,82,72,39,2,45,78,11,1,44,69,64"
            .split(',')
            .map(|char| char.parse::<u16>().unwrap())
            .collect();

    let board_input: Vec<Vec<Vec<u8>>> = raw_input
        .split("\n\n")
        .map(|data| {
            data.split('\n')
                .map(|line| {
                    line.split(" ")
                        .filter(|a| a != &"")
                        .map(|b| b.parse::<u8>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    part1(&board_input, &instructions);
    part2(&board_input, &instructions);
}

#[derive(Clone, Debug)]
struct Board {
    board: Vec<Vec<Number>>,
    has_won: bool,
    won_index: u16,
}

impl Board {
    pub fn mark_number(&mut self, num: u16) -> bool {
        for (row, numbers) in self.board.clone().iter().enumerate() {
            for (column, number) in numbers.iter().enumerate() {
                if number.value != num as u8 {
                    continue;
                }
                self.board[row][column].marked = true;

                if self.board[row].iter().all(|it| it.marked)
                    || self.board.iter().all(|it| it[column].marked)
                {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn print_answer(&self, last_call: u16) {
        let mut unmarked_count: u16 = 0;
        for line in &self.board {
            for num in line {
                if !num.marked {
                    unmarked_count += num.value as u16;
                }
            }
        }
        println!(
            "Answer: \x1b[1;31m {}\x1b[0m",
            unmarked_count as u32 * last_call as u32
        );
    }

    pub fn score(&self, last_call: u16) -> u32 {
        let mut unmarked_count: u16 = 0;
        for line in &self.board {
            for num in line {
                if !num.marked {
                    unmarked_count += num.value as u16;
                }
            }
        }
        return unmarked_count as u32 * last_call as u32;
    }

    pub fn print(&self) {
        for line in &self.board {
            for char in line {
                let char_value: String;
                if char.value < 10 {
                    char_value = format!("0{}", char.value)
                } else {
                    char_value = format!("{}", char.value)
                }

                if char.marked {
                    print!("\x1b[1m\x1b[1;31m {}\x1b[0m", char_value);
                } else {
                    print!(" {}", char_value);
                }
            }
            print!("\n")
        }
        println!();
    }
}

#[derive(Clone, Copy, Debug)]
struct Number {
    value: u8,
    marked: bool,
}

pub fn part1(board_input: &Vec<Vec<Vec<u8>>>, instructions: &Vec<u16>) {
    let mut boards: Vec<Board> = vec![];

    // Create the new boards and store them in the <boards vector>
    for board_iter in board_input {
        let mut board_iter_number: Vec<Vec<Number>> = vec![];
        for line in board_iter {
            let mut board_iter_char_number: Vec<Number> = vec![];
            for char in line {
                board_iter_char_number.push(Number {
                    value: *char,
                    marked: false,
                })
            }
            board_iter_number.push(board_iter_char_number);
        }

        boards.push(Board {
            board: board_iter_number,
            has_won: false,
            won_index: 0,
        });
    }

    for instruction in instructions {
        for board_index in 0..boards.len() {
            if boards[board_index].mark_number(*instruction) {
                println!(
                    "\x1b[1;32m Found first board: \x1b[1;33m{}\x1b[0m",
                    board_index + 1
                );
                boards[board_index].print();
                boards[board_index].print_answer(*instruction as u16);
                return;
            }
        }
    }
}

pub fn part2(board_input: &Vec<Vec<Vec<u8>>>, instructions: &Vec<u16>) {
    let mut boards: Vec<Board> = vec![];

    // Create the new boards and store them in the <boards vector>
    for board_iter in board_input {
        let mut board_iter_number: Vec<Vec<Number>> = vec![];
        for line in board_iter {
            let mut board_iter_char_number: Vec<Number> = vec![];
            for char in line {
                board_iter_char_number.push(Number {
                    value: *char,
                    marked: false,
                })
            }
            board_iter_number.push(board_iter_char_number);
        }

        boards.push(Board {
            board: board_iter_number,
            has_won: false,
            won_index: 0,
        });
    }

    // let mut winner_boards: Vec<Board> = vec![];

    for instruct_index in 0..instructions.len() {
        let instruction = instructions[instruct_index];

        for board in boards.iter_mut() {
            if board.mark_number(instruction) {
                board.has_won = true;
                board.won_index = instruct_index as u16;
            }
        }

        let mut highest_won_index = 0;
        if boards.iter().all(|it| it.has_won) {
            for board in boards.iter_mut() {
                if board.has_won {
                    if board.won_index > highest_won_index {
                        highest_won_index = board.won_index;
                    }
                }
            }
            for board in boards {
                if board.won_index == highest_won_index {
                    println!("Score {:?}", board.score(instruction));
                    println!("Won Index: {}", board.won_index);
                    board.print();
                }
            }
            return;
        }
    }
}