
fn main() {

    let blocks = include_str!("./input.txt").split("\n\n").collect::<Vec<&str>>();

    let mut counter: usize = 0;
    for block in blocks {
        //println!("---------------------------------------");
        let inverted_block = cols_to_lines(block);
        
        let mut last_line = "";
        for (idx, line) in block.lines().enumerate() {
            if line.eq(last_line) && is_mirror(&block, idx) {
                counter += 100 * idx;
                println!("{} lines above the mirrored line", idx);
                break;
            }
            last_line = line;
        }
        for (idx, line) in inverted_block.lines().enumerate() {
            if line.eq(last_line) && is_mirror(&inverted_block, idx){
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

    let mut i = 1;

    while idx + i < lines.len() && (idx as i8) - i as i8 - 1 >= 0 {
        if lines[idx+i].eq(lines[idx-i-1]) {
            //println!("{} = {}", lines[idx+i], lines[idx-i-1]);
            i+= 1;
        } else {
            //println!("{} != {}", lines[idx+i], lines[idx-i-1]);
            return false;
        }
    }
    true
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