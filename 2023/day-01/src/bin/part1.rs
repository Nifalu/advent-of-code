use aoc_utils_by_nifalu::*;

fn main() {
    let contents = include_str!("./input.txt");

    let mut sum: i32 = 0;

    for line in contents.lines() {
        sum += line.retrieve_first_digit().unwrap_or(0);
        sum += line.retrieve_last_digit().unwrap_or(0);
    }

    println!("{}", sum);
}