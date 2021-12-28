use crate::functions::lines_from_file;
use std::collections::HashMap;
use std::slice::SliceIndex;

struct board_number {
    value: i32,
    called: bool,
}

struct board {
    line1: Vec<board_number>,
    line2: Vec<board_number>,
    line3: Vec<board_number>,
    line4: Vec<board_number>,
    line5: Vec<board_number>,
}

pub fn day_four() {
    let lines = lines_from_file("src/day_four_data");
    let mut called_numbers = "";
    let mut boards: Vec<Vec<&str>> = Vec::new();
    let mut board_count = 0;

    //Read first line into called numbers
    for index in 0..1 {
        called_numbers = &lines[index];
    }
    // Verify we are getting the first line
    println!("Called Numbers: {}", called_numbers);

    // Read data into vectors
    /*let mut index = 0;
    let size: usize = 601;
    while &index < &size {
        if lines[&index].is_empty() {
            board_count += 1;
            &index += 1;
        }
        else {
            boards[&index].line1[0] = board_number(&lines[index]);
            boards
        }
    }*/
    let mut b = HashMap::new();
    b.insert(1, (1, 2, 3, 4, 5));
    b.insert(2, (1, 2, 3, 4, 5));
    b.insert(3, (1, 2, 3, 4, 5));
    b.insert(4, (1, 2, 3, 4, 5));
    b.insert(5, (1, 2, 3, 4, 5));
    // Verify hash map worked correctly
    println!("Hash Map: {:?}", b);

    let mut indie = 0;
    for index in 1..601 {
        if !lines[index].is_empty() {
            let mut row: Vec<&str> = Vec::new();
            let s = lines[index].split(",");
            for st in s {
                row.push(st);
            }
            // Verify row process
            println!("row: {:?}", row);
            boards.push(row);
        }
        else {indie += 1;}
    }
    // Verify boards function
    println!("boards: {:?}", boards);
    //println!("Indie: {}", indie);
}
