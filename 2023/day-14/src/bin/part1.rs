
fn main() {
    let mut grid: Vec<Vec<char>> = Vec::new();
    include_str!("./test.txt").lines()
    .for_each(|line: &str| {
        let mut vec: Vec<char> = Vec::new();
        line.chars().for_each(|c| vec.push(c));
        grid.push(vec);
    });

    let mut counter = 0;

    for i in 0..grid[0].len() {
        if grid[0][i].eq(&'O') {
            counter += grid.len();
        }
    }


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
                counter += grid.len() - (curr_row);
                //println!("+ {}", grid.len() - (curr_row));
            }
        }
    }

    println!("Total: {}", counter);
    
    for line in grid {
        println!("{:?}", line);
    }

}