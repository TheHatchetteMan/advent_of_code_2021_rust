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
