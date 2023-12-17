
fn main() {
    let mut grid: Vec<Vec<char>> = Vec::new();
    include_str!("./input.txt").lines()
    .for_each(|line: &str| {
        let mut vec: Vec<char> = Vec::new();
        line.chars().for_each(|c| vec.push(c));
        grid.push(vec);
    });

    let mut last_i = 0;
    for i in 0..1010 {
        
        tilt(&mut grid);
        grid = rotate(&mut grid);
        tilt(&mut grid);
        grid = rotate(&mut grid);
        tilt(&mut grid);
        grid = rotate(&mut grid);
        tilt(&mut grid);
        grid = rotate(&mut grid);
        println!("{}", calc_load(&mut grid));
        if calc_load(&mut grid) == 112491 {
            println!("loop {}", i - last_i);
            last_i = i;
        };
        
    }
}


fn tilt(grid: &mut Vec<Vec<char>>) {
    for row in 1..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col].eq(&'O') {
                grid[row][col] = '.';
                let mut curr_row: usize = row;
                while grid[curr_row-1][col].eq(&'.') {
                    if curr_row - 1 == 0 {
                        curr_row -= 1;
                        break;
                    };
                    curr_row -= 1;
                }
                grid[curr_row][col] = 'O';
            }
        }
    }
}

fn calc_load(grid: &mut Vec<Vec<char>>) -> i32 {
    let mut counter = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col].eq(&'O') {
                counter += grid.len() - row;
            }
        }
    }
    counter as i32
}

fn rotate(grid: &Vec<Vec<char>>) -> Vec<Vec<char>>{
    let mut new_grid: Vec<Vec<char>> = grid.clone();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            new_grid[j][grid.len() -i -1] = grid[i][j];
        }
    }
    new_grid
}
