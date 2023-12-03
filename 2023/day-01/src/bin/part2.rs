use std::collections::HashMap;
use aoc_utils_by_nifalu::*;

fn main() {
    let contents = include_str!("./input.txt");
    let mut only_strings = contents.to_string();
    
    let mut sum: i32 = 0;

    let replacements: Vec<(&str, &str)> = vec![
        ("0", "zero"), ("1", "one"), ("2", "two"), ("3", "three"), ("4", "four"), ("5", "five"), ("6", "six"), ("7", "seven"), ("8", "eight"), ("9", "nine")
        ];
    
    let num_strings: Vec<&str> = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let patterns: HashMap<&str, i32> = [
        ("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4),
        ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
    ].iter().cloned().collect();

    for (num, num_string) in &replacements {
        only_strings = only_strings.replace(num, num_string);
    }


    for line in only_strings.lines() {
        println!("{}", line);
        let positions = line.find_positions(&num_strings);
        
        if let Some(first_position) = positions.iter().min_by_key(|&(i, _)| i) {
            sum += patterns.get(&first_position.1.as_str()).unwrap_or(&0) * 10;
        };

        if let Some(last_position) = positions.iter().max_by_key(|&(i, _)| i) {
            sum += patterns.get(&last_position.1.as_str()).unwrap_or(&0);
        }
    }

    println!("{}", sum);
}