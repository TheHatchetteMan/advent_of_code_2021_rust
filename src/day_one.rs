use crate::functions::lines_from_file;

pub fn day_one() {
    let lines = lines_from_file("src/day_one_data");
    let lines_length = lines.len();
    let mut index = 0;
    let mut count = 0;
    while index < lines_length - 1 {
        if lines[index + 1] > lines[index] {
            count += 1;
        }
        index += 1;
    }
    println!("Iterated Comparison Count: {}", count);
}

// Attempts:
// ---------------
// 1446 is too low
// 1447 is too low
// Correct answer = 1448
// Never did actually get the code working right, it only counts 1447 and then goes out of range

// Part 2
pub fn day_one_2() {
    let lines = lines_from_file("src/day_one_data");
    let lines_length = lines.len();
    let mut index = 0;
    let mut count = 0;

    while index < lines_length - 3 {
        let win1 = lines[index].parse::<i32>().unwrap()
            + lines[index + 1].parse::<i32>().unwrap()
            + lines[index + 2].parse::<i32>().unwrap();
        let win2 = lines[index + 1].parse::<i32>().unwrap()
            + lines[index + 2].parse::<i32>().unwrap()
            + lines[index + 3].parse::<i32>().unwrap();
        if win2 > win1 {
            count += 1
        }
        index += 1;
    }
    println!("Window Increase Count: {}", count);
}

// Attempts:
// ----------------
// 1471 is the correct answer
