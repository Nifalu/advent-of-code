

fn main() {
    let contents = include_str!("./input.txt");

    let mut iter:std::str::Split<'_, &str> = contents.split("\n\n");

    let input: Vec<i64> = iter
    .next()
    .and_then(|i: &str| Some(find_all_numbers(i)))
    .unwrap_or_else(Vec::new);

    let maps: Vec<Vec<[i64; 3]>> = iter
    .map(|s| 
        find_all_numbers(s)
            .chunks(3) // Create chunks of 3
            .map(|chunk| {
                let arr = [chunk[0], chunk[1], chunk[2]]; // Create an array from the chunk
                arr
            })
            .collect() // Collect chunks into a vector
    )
    .collect(); // Collect these vectors into the outer vector

    
    let mut seeds: Vec<(i64, i64)> = Vec::new(); // seed = (lower_bound, upper_bound)

    for seed in input.chunks(2) {
        seeds.push((seed[0], seed[0] + seed[1]))
    }

    println!("seeds: {:?} \n maps: {:?}", seeds, maps);

}

fn find_all_numbers(s: &str) -> Vec<i64> {
    s.split_whitespace()
        .filter_map(|word: &str| word.parse::<i64>().ok())
        .collect()
}

