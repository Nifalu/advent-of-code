

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
enum Directions {
    NORTH,
    SOUTH,
    WEST,
    EAST,
}

fn main() {
    let raw_input = include_str!("./input.txt");

    let mut sequences: Vec<Vec<char>> = Vec::new();
    raw_input.lines()
    .for_each(|line: &str| {
        sequences.push(line.chars().collect::<Vec<char>>())
    });

    let mut clean_map: Vec<Vec<char>> = Vec::new();
    for i in 0..sequences.len() {
        clean_map.push(Vec::new());
        for _ in 0..sequences[0].len() {
            clean_map[i].push(' ');
        };
    }

    let mut curr_pos = ((raw_input.find("S").unwrap() as i32 / (sequences[0].len() as i32 +1)),
                            (raw_input.find("S").unwrap() as i32 % (sequences[0].len() as i32 +1)));
    
    assert_eq!(sequences[curr_pos.0 as usize][curr_pos.1 as usize], 'S');
    
    let mut curr_dir = Directions::NORTH;
    let mut distance = 0;

    let neighbors: [((i32, i32), Directions); 4] = [((-1, 0), Directions::NORTH),
                                    ((0, 1), Directions::EAST),
                                    ((1, 0), Directions::SOUTH),
                                    ((0, -1), Directions::WEST)];
    for ((off_x, off_y), dir) in neighbors {
        let c = sequences[(curr_pos.0 + off_x) as usize][(curr_pos.1 + off_y) as usize];
        match dir {
            Directions::NORTH => {
                if c == '|' || c == '7' || c == 'F' {
                    //println!("Figure out starting direction {:?}", sequences[curr_pos.0 as usize][curr_pos.1 as usize]);
                    (curr_pos, curr_dir) = find_next_step(curr_pos.0, curr_pos.1, &dir, &sequences, &mut clean_map);
                    break;
                }
            }
            Directions::EAST => {
                if c == '-' || c == '7' || c == 'J' {
                    //println!("Figure out starting direction {:?}", sequences[curr_pos.0 as usize][curr_pos.1 as usize]);
                    (curr_pos, curr_dir) = find_next_step(curr_pos.0, curr_pos.1, &dir, &sequences, &mut clean_map);
                    break;
                } 
            }
            Directions::SOUTH => {
                if c == '|' || c == 'L' || c == 'J' {
                    //println!("Figure out starting direction {:?}", sequences[curr_pos.0 as usize][curr_pos.1 as usize]);
                    (curr_pos, curr_dir) = find_next_step(curr_pos.0, curr_pos.1, &dir, &sequences, &mut clean_map);
                    break;
                } 
            }
            Directions::WEST => {
                if c == '-' || c == 'L' || c == 'F' {
                    //println!("Figure out starting direction {:?}", sequences[curr_pos.0 as usize][curr_pos.1 as usize]);
                    (curr_pos, curr_dir) = find_next_step(curr_pos.0, curr_pos.1, &dir, &sequences, &mut clean_map);
                    break;
                } 
            }
        }
        }
    while sequences[curr_pos.0 as usize][curr_pos.1 as usize] != 'S' {
        (curr_pos, curr_dir) = find_next_step(curr_pos.0, curr_pos.1, &curr_dir, &sequences, &mut clean_map);
        distance += 1;
    }

    println!("Distance: {}", distance / 2 + 1); 
    let mut sum = 0;
    for line in clean_map {
        sum += count_insides(&line)
    }
    println!("Insides: {}", sum);
}


fn find_next_step(x: i32, y: i32, direction: &Directions, sequences: &Vec<Vec<char>>, clean_map: &mut Vec<Vec<char>>) -> ((i32, i32), Directions) {

    if sequences[x as usize][y as usize].eq(&'S') {
        clean_map[x as usize][y as usize] = 'F'
    } else {
        clean_map[x as usize][y as usize] = sequences[x as usize][y as usize];
    }


    //println!("walking over: {:?} looking {:?}", sequences[x as usize][y as usize], direction);
    let mut new_x = x;
    let mut new_y = y;
    match direction {
        Directions::NORTH => new_x -= 1,
        Directions::EAST  => new_y += 1,
        Directions::SOUTH => new_x += 1,
        Directions::WEST  => new_y -= 1,
    }
    

    let cell = sequences[new_x as usize][new_y as usize];
    let new_d = match cell {
        'L' => match direction {
            Directions::WEST => Directions::NORTH,
            _                => Directions::EAST,
        }
        ,
        'J' => match direction {
            Directions::EAST => Directions::NORTH,
            _                => Directions::WEST,
        },
        'F' => match direction {
            Directions::WEST => Directions::SOUTH,
            _                => Directions::EAST,
        },
        '7' => match direction {
            Directions::EAST => Directions::SOUTH,
            _                => Directions::WEST,
        },
        _ => direction.clone()
    };  
    return ((new_x, new_y), new_d)
}


fn count_insides(line: &Vec<char> ) -> i32 {

    let mut came_from: Directions = Directions::NORTH;
    let mut counter: i32 = 0;
    let mut inside: bool = false;

    for c in line {
        if inside && c.eq(&' ') {
            counter += 1;
            print!("#");
        } else if c.eq(&' ') {
            print!(" ");
        } else {
            print!("{}",c);
        }

        if c.eq(&'L') {
            came_from = Directions::NORTH
        } else if c.eq(&'F') {
            came_from = Directions::SOUTH
        } else if c.eq(&'J') && came_from.eq(&Directions::SOUTH) || 
            c.eq(&'7') && came_from.eq(&&Directions::NORTH) ||
            c.eq(&'|') {
            inside = !inside;
        }
    }
    println!();
    counter
}