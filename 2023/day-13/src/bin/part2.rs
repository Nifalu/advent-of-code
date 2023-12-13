

fn main() {

    let blocks = include_str!("./input.txt").split("\n\n").collect::<Vec<&str>>();

    let mut counter: usize = 0;
    for block in blocks {
        //println!("---------------------------------------");
        let inverted_block = cols_to_lines(block);
        
        let mut last_line = "";
        for (idx, line) in block.lines().enumerate() {
            if (is_one_bit_difference(line, last_line) || line.eq(last_line)) 
            && is_mirror(&block, idx) {
                counter += 100 * idx;
                println!("{} lines above the mirrored line", idx);
                break;
            }
            last_line = line;
        }
        for (idx, line) in inverted_block.lines().enumerate() {
            if (is_one_bit_difference(line, last_line) || line.eq(last_line)) 
            && is_mirror(&inverted_block, idx) {
                counter += idx;
                println!("{} lines left to the mirrored line", idx);
                break;
            }
            last_line = line;
        }
        println!("---------------------------------------");
    }
    println!("{}", counter);
}


fn is_mirror(block: &str, idx: usize) -> bool {

    let mut lines: Vec<&str> = Vec::new();
    block.lines().for_each(|line| lines.push(line));
    let mut had_difference: bool = false;
    let mut i = 0;

    while idx + i < lines.len() && (idx as i8) - i as i8 - 1 >= 0 {
        if lines[idx+i].eq(lines[idx-i-1])
        || is_one_bit_difference(lines[idx+i], lines[idx-i-1]) {

            if is_one_bit_difference(lines[idx+i], lines[idx-i-1]) && !had_difference {
                had_difference = true;
            }
            //println!("{} = {}", lines[idx+i], lines[idx-i-1]);
            i += 1;
        } else {
            //println!("{} != {}", lines[idx+i], lines[idx-i-1]);
            return false;
        }
    }
    //println!("Block\n{}\nDoes mirror? {}\nAt Index {}", block, had_difference, idx);
    had_difference
    
}


fn cols_to_lines(block: &str) -> String {

    let mut inverted_block: Vec<Vec<char>> = Vec::new();

    block.lines().for_each(|line| {
        let mut vec: Vec<char> = Vec::new();
        line.chars().for_each(|char| vec.push(char));
        inverted_block.push(vec);
    });

    let mut tmp: Vec<Vec<char>> = Vec::new();
    for i in 0..inverted_block[0].len() {
        let mut vec: Vec<char> = Vec::new();
        for j in 0..inverted_block.len() {
            vec.push(inverted_block[j][i])
        }
        tmp.push(vec);
    }

    let inverted_block = tmp
    .into_iter()
    .map(|vec| vec.into_iter().collect::<String>())
    .collect::<Vec<String>>()
    .join("\n");

    inverted_block
}


fn to_bitmap(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| (acc << 1) | if c == '#' { 1 } else { 0 })
}

fn is_one_bit_difference(s1: &str, s2: &str) -> bool {
    let bitmap1 = to_bitmap(s1);
    let bitmap2 = to_bitmap(s2);
    let xor = bitmap1 ^ bitmap2;

    xor != 0 && (xor & (xor - 1)) == 0
}