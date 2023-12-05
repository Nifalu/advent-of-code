


fn main() {
    
    let contents = include_str!("./input.txt");
    let (seeds, maps) = contents.split_once(":\n").unwrap();

    // Get the seed numbers as u64
    let seeds: Vec<i64> = find_all_numbers(seeds);

    // Convert the maps to integers
    let string_maps: Vec<&str> = maps.split(":").collect::<Vec<&str>>();
    let mut maps: Vec<Vec<i64>> = Vec::new();
    for line in string_maps {
        maps.push(find_all_numbers(line));
    }

    let mut shortest_distance: i64 = i64::MAX;

    for s in seeds {
        let mut seed = s;
        for map in &maps {
            seed = apply_mapping(seed, map);
        }
        println!{"{} has distance: {}", s, seed};
        shortest_distance = shortest_distance.min(seed);
        println!("SHORTEST DISTANCE : {} ", shortest_distance);
    }

}

fn apply_mapping(seed: i64, map: &Vec<i64>) -> i64 {

    for (idx, _) in map.iter().enumerate() {
        if idx % 3 == 0 {                                   // every chunk of size 3 equals one mapping.
            let destination = map[idx];
            let origin = map[idx+1];
            let range = map[idx+2];
            if seed >= origin && seed < origin + range {   // check if seed is in range
                return seed + (destination - origin);       // map seed to new value
            }
        }
    }
    //println!("Seed is now: {}", seed);
    return seed; // return unchanged seed if no mapping was applied

}

fn find_all_numbers(s: &str) -> Vec<i64> {
    s.split_whitespace()
        .filter_map(|word: &str| word.parse::<i64>().ok())
        .collect()
}

