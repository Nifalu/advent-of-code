use aoc_utils_by_nifalu::{self, RetrieveInts};



fn main() {
    let contents = include_str!("./input.txt");

    let mut sum: i32 = 0;

    let grid: Vec<Vec<char>> = contents
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect();

    for row in 0..grid.len() {
        for col in 0..grid.get(0).unwrap().len() {
            
            if !grid[row][col].is_digit(10) && grid[row][col] != '.' {
                sum += find_neighbours(&grid, row as isize, col as isize);
            }
        }
    }
    println!("{}", sum);
}

fn find_neighbours(grid: &Vec<Vec<char>>, row: isize, col: isize) -> i32 {
    let offsets = [
    [-1, -1], [-1, 0], [-1, 1],
    [0, -1], [0, 1],
    [1, -1], [1, 0], [1, 1]
    ];

    let mut neighbors_sum: i32 = 0;
    let mut last_number: i32 = 0;
    for offset in offsets.iter() {
        let new_row = (row + offset[0]) as usize;
        let new_col = (col + offset[1]) as usize;
        if grid[new_row][new_col].is_digit(10) {
            let number = grid.get(new_row).unwrap().retrieve_int_at_pos(new_col).unwrap_or(0);
            if number != last_number {
                neighbors_sum += number;
                println!("neighbors found {}", number);
            }
            last_number = number;
        }
    }

    neighbors_sum
}