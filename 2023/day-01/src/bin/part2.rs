use std::collections::HashMap;


fn check_for_pattern(patterns: &HashMap<&str, u32>, s: &str) -> (bool, u32) {
    for (num_string, num) in patterns {
        if s.starts_with(num_string) {
            return (true, *num);
        }
    }
    return (false, 0);
}

fn check_for_digit(c: char) -> (bool, u32) {
    if c.is_ascii_digit() {
        return (true, c.to_digit(10).unwrap());
    }
    return (false, 0);
}


fn main() {
    let contents = include_str!("./input.txt");
    
    let mut sum: u32 = 0;

    let patterns: HashMap<&str, u32> = [
        ("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4),
        ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
    ].iter().cloned().collect();

    for line in contents.lines() {

        // Iterate from the start
        for (idx, c) in line.chars().enumerate() {

            let (is_digit, value) = check_for_digit(c);
            let (is_num_string, value1) = check_for_pattern(&patterns, &line[idx..]);

            if is_digit || is_num_string {
                sum += (value + value1) * 10;
                break;
            }
        }

        // String
        for (idx, c) in line.chars().rev().enumerate() {
            let rev_idx = line.len() - idx -1;

            let (is_digit, value) = check_for_digit(c);
            let (is_num_string, value1) = check_for_pattern(&patterns, &line[rev_idx..]);

            if is_digit || is_num_string {
                sum += value + value1;
                break;
            }
        }
    }

    println!("{}", sum);
}