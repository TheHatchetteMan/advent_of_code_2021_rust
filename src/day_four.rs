use crate::functions::lines_from_file;
use std::collections::HashMap;
use std::slice::SliceIndex;

pub fn day_four() {
    // Read in the lines from the file
    let mut lines = lines_from_file("src/day_four_data");

    // Read the first line into choices, these are the called bingo numbers
    let choices = lines
        .drain(0..1)
        .collect::<String>()
        .split(',')
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

    // Read the remaining lines into chunks of 5 to indicate the boards
    let mut boards = lines
        .as_slice()
        .chunks(5)
        .map(|chunk| {
            let mut board = HashMap::new();
            for (y, line) in chunk.iter().enumerate() {
                for (x, num) in line.split(' ').filter(|c| !c.is_empty()).enumerate() {
                    board.insert((x, y), (num, false));
                }
            }

            board
        })
        .collect::<Vec<_>>();

    // Determine the winning board
    for choice in choices {
        for board in boards.iter_mut() {
            for cell in board.iter_mut() {
                if cell.1 .0  == choice {
                    cell.1 .1 = true;
                }
            }

            let mut winner = false;
            for y in 0..5 {
                let mut row = true;
                for x in 0..5 {
                    row = row && board[&(x, y)].1
                }
                if row {
                    winner = true;
                    break;
                }
            }

            for x in 0..5 {
                let mut col = true;
                for y in 0..5 {
                    col = col && board[&(x, y)].1
                }
                if col {
                    winner = true;
                    break;
                }
            }

            if winner {
                let sum = board
                    .values()
                    .filter(|v| !v.1)
                    .map(|v| v.0.parse::<u64>().unwrap())
                    .sum::<u64>();

                println!("Bingo Score: {}", sum * (choice.parse::<u64>().unwrap()));

                return;
            }
        }
    }
}

pub fn day_four_2 () {
    // Read in the lines from the file
    let mut lines = lines_from_file("src/day_four_data");

    // Read the first line into choices, these are the called bingo numbers
    let choices = lines
        .drain(0..1)
        .collect::<String>()
        .split(',')
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();

    // Read the remaining lines into chunks of 5 to indicate the boards
    let mut boards = lines
        .as_slice()
        .chunks(5)
        .map(|chunk| {
            let mut board = HashMap::new();
            for (y, line) in chunk.iter().enumerate() {
                for (x, num) in line.split(' ').filter(|c| !c.is_empty()).enumerate() {
                    board.insert((x, y), (num, false));
                }
            }

            board
        })
        .collect::<Vec<_>>();

    let mut won = vec![false; boards.len()];
    for choice in choices {
        for (board_num, board) in boards.iter_mut().enumerate() {
            for cell in board.iter_mut() {
                if cell.1 .0 == choice {
                    cell.1 .1 = true;
                }
            }

            let mut winner = false;
            for y in 0..5 {
                let mut row = true;
                for x in 0..5 {
                    row = row && board[&(x, y)].1
                }
                if row {
                    winner = true;
                    break;
                }
            }

            for x in 0..5 {
                let mut col = true;
                for y in 0..5 {
                    col = col && board[&(x, y)].1
                }
                if col {
                    winner = true;
                    break;
                }
            }

            if winner {
                if !won[board_num] && won.iter().filter(|p| !**p).count() == 1 {
                    let sum = board
                        .values()
                        .filter(|v| !v.1)
                        .map(|v| v.0.parse::<u64>().unwrap())
                        .sum::<u64>();

                    println!("{}", sum * (choice.parse::<u64>().unwrap()));

                    return;
                } else {
                    won[board_num] = true;
                }
            }
        }
    }
}
