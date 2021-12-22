use crate::functions::lines_from_file;

pub fn day_three() {
    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    let lines = lines_from_file("src/day_three_data");
    //let line_length = &lines[0].len();
    let mut index = 0;

    while index < 12 {
        let mut binary_one = 0;
        let mut binary_zero = 0;
        for line in &lines {
            match line.as_bytes()[index] {
                // UTF-8 "1"
                49 => binary_one += 1,
                //UTF-8 "0"
                48 => binary_zero += 1,
                _ => println!("Something Wrong"),
            }
        }
        if binary_one > binary_zero {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }

        index += 1;
    }
    println!("Gamma: {} and Epsilon: {}", gamma, epsilon);
    let int_gamma = isize::from_str_radix(&gamma, 2).unwrap();
    let int_epsilon = isize::from_str_radix(&epsilon, 2).unwrap();
    println!(
        "Gamma: {} and Epsilon: {} Multiplied: {}",
        int_gamma,
        int_epsilon,
        int_gamma * int_epsilon
    );
}

// Correct answer first try

// Part 2

pub fn day_three_2() {
    let mut oxygen_rating: Vec<String> = lines_from_file("src/day_three_data");
    let mut co2_rating: Vec<String> = lines_from_file("src/day_three_data");

    let mut oxy_index = 0;
    let mut co2_index = 0;

    while oxy_index < 12 {
        if oxygen_rating.len() == 1 {
            break;
        };
        let mut binary_one = 0;
        let mut binary_zero = 0;

        for line in &mut oxygen_rating {
            match line.as_bytes()[oxy_index] {
                49 => binary_one += 1,  // UTF-8 "1"
                48 => binary_zero += 1, //UTF-8 "0"
                _ => println!("Something Wrong with assignment oxygen"),
            }
        }
        if binary_one > binary_zero {
            oxygen_rating.retain(|x| x.as_bytes()[oxy_index] == 49);
        } else if binary_zero > binary_one {
            oxygen_rating.retain(|x| x.as_bytes()[oxy_index] == 48);
        } else if binary_one == binary_zero {
            oxygen_rating.retain(|x| x.as_bytes()[oxy_index] == 49);
        } else {
            println!("something wrong with retain oxygen");
        }
        oxy_index += 1;
    }
    while co2_index < 12 {
        if co2_rating.len() == 1 {
            break;
        };
        let mut binary_one = 0;
        let mut binary_zero = 0;

        for line in &mut co2_rating {
            match line.as_bytes()[co2_index] {
                49 => binary_one += 1,  // UTF-8 "1"
                48 => binary_zero += 1, //UTF-8 "0"
                _ => println!("Something Wrong with assignment co2"),
            }
        }
        if binary_one > binary_zero {
            co2_rating.retain(|x| x.as_bytes()[co2_index] == 48);
        } else if binary_zero > binary_one {
            co2_rating.retain(|x| x.as_bytes()[co2_index] == 49);
        } else if binary_one == binary_zero {
            co2_rating.retain(|x| x.as_bytes()[co2_index] == 48);
        } else {
            println!("something wrong with retain co2");
        }
        co2_index += 1;
    }
    println!("Oxygen: {:?} - Co2: {:?}", oxygen_rating, co2_rating);
    let int_oxy = isize::from_str_radix(&oxygen_rating[0], 2).unwrap();
    let int_co2 = isize::from_str_radix(&co2_rating[0], 2).unwrap();
    println!(
        "Oxygen: {} - Co2: {} - Multiplied: {}",
        int_oxy,
        int_co2,
        int_oxy * int_co2
    );

    // Attempts
    // 16719921 is too high
    // 7863147 correct answer
}
