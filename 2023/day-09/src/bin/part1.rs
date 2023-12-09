fn main() {
    // Read Input
    let mut sequences: Vec<Vec<i64>> = Vec::new();
    include_str!("./input.txt").lines()
    .for_each(|line: &str| {
        sequences.push(line.split_whitespace()
        .filter_map(|num: &str| num.parse::<i64>().ok()).collect::<Vec<i64>>());
    });

    let mut sum = 0;

    for sequence in sequences {
        //println!("{:?}", get_left_edge(&sequence));
        sum += calc_next_value(&sequence);
    }

    println!("{}", sum);
    
}


fn get_left_edge(sequence: &Vec<i64>) -> Vec<i64> {
    let mut left_edge: Vec<i64> = Vec::new();
    
    for i in 0..sequence.len() {
        let mut sum: i64 = 0;

        for j in 0..=i {
            sum += (-1 as i64).pow( j as u32) * binomi(i,j) * sequence[i-j];
        }
        
        left_edge.push(sum);
    }
    left_edge
}

fn calc_next_value(sequence: &Vec<i64>) -> i64 {
    let left_edge: Vec<i64> = get_left_edge(&sequence);
    let mut sum = *sequence.last().unwrap();

    for (idx, num) in left_edge.iter().skip(1).enumerate() {
        sum += num * binomi(left_edge.len() -1, idx);
        //println!("num: {}, binomi {}, idx: {}", num, binomi(left_edge.len()-1, idx), idx);
    }
    
    sum
}

fn binomi(n: usize, k: usize) -> i64 {
    if (n == k) || (k == 0) {
        return 1
    }
    return binomi(n - 1, k) + binomi(n - 1, k - 1);
}