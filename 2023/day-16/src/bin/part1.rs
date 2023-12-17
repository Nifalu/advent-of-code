
static mut INPUT: Vec<Vec<char>> = Vec::new();
static mut LAVA: Vec<Vec<String>> = Vec::new();

const NORTH: (i8, i8) = (-1, 0);
const EAST: (i8, i8) = (0, 1);
const SOUTH: (i8, i8) = (1, 0);
const WEST: (i8, i8) = (0, -1);

fn main() {

    include_str!("./input.txt")
    .lines()
    .for_each(|line: &str| {
        let mut cs: Vec<char> = Vec::new();
        let mut lava1: Vec<String> = Vec::new();
        line.chars().for_each(|c: char| {
            cs.push(c);
            lava1.push(".".to_string());
        });
        unsafe { INPUT.push(cs) };
        unsafe { LAVA.push(lava1) };
    });
    // println!("{}, {}\n{}, {}", input.len(), input[0].len(), lava_map.len(), lava_map[0].len());

    
    forward((0,0), EAST);
    print_clean_lava();
    println!("{}",count_lava());
}


fn forward((posx, posy): (i8, i8), (x, y): (i8, i8)) {
    
    let mut curr_field: (i8, i8) = (posx, posy);
    let mut next_field: (i8, i8);

    // Check if we are out of bounds or if we have already been here
    if !is_inside(curr_field) || get_lava(curr_field).contains(get_direction((x, y)).as_str()) {
        return;
    }

    // Walk straight until reaching some obstacle or boundary.
    next_field = (curr_field.0 + x, curr_field.1 + y);
    while is_inside(next_field) && (get_field(curr_field).eq(&'.')
    || (get_field(curr_field).eq(&'|') && y == 0)       // vertical split while walking vertically
    || (get_field(curr_field).eq(&'-') && x == 0)) {    // horizontal split while walking horizontally
        set_lava(curr_field, (x, y));
        curr_field = next_field;
        next_field = (next_field.0 + x, next_field.1 + y)
    }
    set_lava(curr_field, (x, y));


    if get_field(curr_field).eq(&'-') && y == 0 {
        forward((curr_field.0, curr_field.1 - 1), WEST);
        forward((curr_field.0, curr_field.1 + 1), EAST);
    }

    if get_field(curr_field).eq(&'|') && x == 0 {
        forward((curr_field.0 - 1, curr_field.1), NORTH);
        forward((curr_field.0 + 1, curr_field.1), SOUTH);
    }

    if get_field(curr_field).eq(&'\\') {
        if x == 1 { forward((curr_field.0, curr_field.1 + 1), EAST) }
        if x == -1 { forward((curr_field.0, curr_field.1 - 1), WEST) }
        if y == 1 { forward((curr_field.0 + 1, curr_field.1), SOUTH) }
        if y == -1 { forward((curr_field.0 - 1, curr_field.1), NORTH) }
    }

    if get_field(curr_field).eq(&'/') {
        if x == -1 { forward((curr_field.0, curr_field.1 + 1), EAST) }
        if x == 1 { forward((curr_field.0, curr_field.1 - 1), WEST) }
        if y == -1 { forward((curr_field.0 + 1, curr_field.1), SOUTH) }
        if y == 1 { forward((curr_field.0 - 1, curr_field.1), NORTH) }
    }


}

fn is_inside((x, y): (i8, i8)) -> bool {
   unsafe { x >= 0 && y >= 0 && x < INPUT.len() as i8 && y < INPUT[0].len() as i8 }
}

fn get_field((x, y): (i8, i8)) -> char {
    unsafe {INPUT[x as usize][y as usize]}
}

fn get_lava((x, y): (i8, i8)) -> &'static str {
    unsafe {LAVA[x as usize][y as usize].as_str()}
}

fn set_lava((posx, posy): (i8, i8), (x, y): (i8, i8)) {
    let mut dir: String = get_lava((posx, posy)).to_string();
    if dir.eq(".") { dir = "".to_string()}
    dir += get_direction((x,y)).as_str();
    //println!("set lava to {}", dir);
    unsafe {LAVA[posx as usize][posy as usize] = dir}
}

fn print_clean_lava() {
    for line in unsafe { &LAVA } {
        for c in line {
            if c.eq(".") {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
}

fn get_direction((x, y): (i8, i8)) -> String {
    let mut dir = "".to_string();
    if x == 1 { dir += "s" }
    if x == -1 { dir += "n" }
    if y == 1 { dir += "e"}
    if y == -1 { dir += "w" }
    dir
}

fn count_lava() -> i32 {
    let mut counter: i32 = 0;
    unsafe {
        for line in &LAVA {
            for field in line {
                if field.ne(".") {
                    counter += 1;
                }
            }
        }
    }
    counter
}