

fn main() {
    let instr: Vec<(&str, i64)> = parse_input(include_str!("./input.txt"));

    let mut positive: i64 = 0;
    let mut negative: i64 = 0;
    let mut curr_coord: (i64, i64) = (0, 0);
    let mut next_coord: (i64, i64) = calc_next_coord(instr[0].0, instr[0].1, (0, 0));
    let mut perimeter: i64 = 0;

    for i in 1..instr.len() {
        positive += curr_coord.0 * next_coord.1;
        negative += curr_coord.1 * next_coord.0;
        
        perimeter += (curr_coord.0 - next_coord.0).abs() + (curr_coord.1 - next_coord.1).abs();
        
        curr_coord = next_coord;
        next_coord = calc_next_coord(instr[i].0, instr[i].1, next_coord);
    }
    perimeter += (curr_coord.0 - next_coord.0).abs() + (curr_coord.1 - next_coord.1).abs();

    let enclosed_area = 0.5 * (positive - negative).abs() as f64;
    let trench_area = 0.5 * perimeter as f64;
    let total_area = enclosed_area + trench_area + 1.0;

    println!("Enclosed area: {}", enclosed_area);
    println!("Trench area: {}", trench_area);
    println!("Total area: {}", total_area);
}

fn calc_next_coord(dir: &str, dist: i64, curr_coord: (i64, i64)) -> (i64, i64) {
    match dir {
        "R" => (curr_coord.0 + dist, curr_coord.1),
        "L" => (curr_coord.0 - dist, curr_coord.1),
        "U" => (curr_coord.0, curr_coord.1 + dist),
        "D" => (curr_coord.0, curr_coord.1 - dist),
        _   => panic!("Expected a direction...")
    }
}


fn parse_input(input: &str) -> Vec<(&str, i64)> {
    let mut instruction: Vec<(&str, i64)> = Vec::new();

    input.lines().for_each(|line| {

        let (_, hex) = line.split_once('#').unwrap();
        let mut chars = hex.chars();
    
        let mut length: i64 = 0;
        for _ in 0..5 {
            length = length * 16 + chars.next().unwrap().to_digit(16).unwrap() as i64;
        }

        let direction: &str;

        match chars.next().unwrap() {
            '3' => direction = "U",
            '1' => direction = "D",
            '2' => direction = "L",
            '0' => direction = "R",
            _ => panic!("Expected a 0, 1, 2 or 3..."),
        };

        instruction.push((direction, length));
    });
    instruction
}