use crate::functions::lines_from_file;

pub fn day_two() {
    let mut horizontal = 0;
    let mut depth = 0;

    let lines = lines_from_file("src/day_two_data");

    for line in lines {
        let split = line.split(' ');
        let mut direction = "";
        let mut amount = 0;
        for s in split {
            match s {
                "forward" => direction = "forward",
                "down" => direction = "down",
                "up" => direction = "up",
                _ => amount = s.parse::<i32>().unwrap(),
            }
        }
        match direction {
            "forward" => horizontal += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => continue,
        }
    }

    println!(
        "Horizontal: {} and Depth: {} and Multiplication:{}",
        horizontal,
        depth,
        (horizontal * depth)
    );
}

// Got this code correct the first try

// Part two

pub fn day_two_2() {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    let lines = lines_from_file("src/day_two_data");

    for line in lines {
        let split = line.split(' ');
        let mut direction = "";
        let mut amount = 0;
        for s in split {
            match s {
                "forward" => direction = "forward",
                "down" => direction = "down",
                "up" => direction = "up",
                _ => amount = s.parse::<i32>().unwrap(),
            }
        }
        match direction {
            "forward" => {
                horizontal += amount;
                depth += amount * aim;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => continue,
        }
    }
    println!(
        "Horizontal: {} and Depth: {} and Multiplication:{}",
        horizontal,
        depth,
        (horizontal * depth)
    );
}

// Got this code correct the first try
