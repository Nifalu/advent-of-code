

fn main() {
    let contents = include_str!("./input.txt");

    // Split the input on every double-newline
    let mut iter:std::str::Split<'_, &str> = contents.split("\n\n");

    // Store input (seeds and ranges)
    let input: Vec<i64> = iter
    .next()
    .and_then(|i: &str| Some(find_all_numbers(i)))
    .unwrap_or_else(Vec::new);

    // Store Maps
    let maps: Vec<Vec<(i64, i64, i64)>> = iter
    .map(|s| 
        find_all_numbers(s)
            .chunks(3) // Create chunks of 3
            .map(|chunk| {
                let arr = (chunk[0], chunk[1], chunk[2]); // Create an array from the chunk
                arr
            })
            .collect() // Collect chunks into a vector
    )
    .collect(); // Collect these vectors into the outer vector

    
    let mut seeds: Vec<(i64, i64)> = Vec::new(); // seed = (lower_bound, upper_bound)

    // Store all original seed ranges
    for seed in input.chunks(2) {
        seeds.push((seed[0], seed[0] + seed[1]))
    }

    // Go through each Mapping
    for map in &maps {

        // Store the new ranges that were calculated from the seed ranges
        let mut mapped_ranges: Vec<(i64, i64)> = Vec::new(); 

        // As long as there are seeds...
        while seeds.len() > 0 {
            let (lower, upper) = seeds.pop().unwrap();
            let mut success: bool = false;
            // For each Seed, go through every mapping-range in the current map.
            for (dest, origin, range) in map {
                // Calculate potential overlaps between the seed range and the mapping range
                let overlap_start = lower.max(*origin);
                let overlap_end = upper.min(origin + range);
                // If there is an overlap, something can be mapped...
                if overlap_start < overlap_end {
                    // Save the new mapped range of the overlapping part
                    mapped_ranges.push((overlap_start - origin + dest, overlap_end - origin + dest));
                    // Seed-ranges that were not overlapping are added to the seeds
                    if overlap_start > lower {
                        seeds.push((lower, overlap_start));
                    }
                    if upper > overlap_end {
                        seeds.push((overlap_end, upper));
                    }
                    success = true; // We found a mapping
                    break;
                }
            }
            if !success { // we did not find a mapping
                mapped_ranges.push((lower, upper));
            }
        }
        // Go through the next map with the newly calculated ranges
        seeds = mapped_ranges;
    }

    // The smallest lower bound is our result.
    seeds.sort_by_key(|&(lower, _)| lower);
    println!("Seed: {:?}", seeds[0].0);

}

fn find_all_numbers(s: &str) -> Vec<i64> {
    s.split_whitespace()
        .filter_map(|word: &str| word.parse::<i64>().ok())
        .collect()
}

