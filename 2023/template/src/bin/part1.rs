
fn main() {
    let mut sequences: Vec<Vec<i64>> = Vec::new();
    include_str!("./input.txt").lines()
    .for_each(|line: &str| {
        sequences.push(line.split_whitespace()
        .filter_map(|num: &str| num.parse::<i64>().ok()).collect::<Vec<i64>>());
    });
}