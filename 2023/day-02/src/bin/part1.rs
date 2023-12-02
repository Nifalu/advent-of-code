use regex::Regex;

fn main() {
    let contents = include_str!("./input.txt");

    let re = Regex::new(r"[:;, ]").unwrap();

    let mut sum: i32 = 0;

    for (game,line) in contents.lines().enumerate() {
        let splits: Vec<&str> = re.split(line).collect();

        let digits: Vec<(usize, i32)> = splits
        .iter()
        .enumerate()
        .filter_map(|(index, s)| match s.parse::<i32>() {
            Ok(num) => Some((index, num)),
            Err(_) => None,
        })
        .collect();

        let mut isPossible = true;

        for (idx, digit) in digits {
            //println!("game {} has digit {} and {} at index {}",game+1, digit, splits.get(idx + 1).unwrap().to_string(), idx);
            if idx != 1 { // Game Number

                let color = splits.get(idx + 1);

                if color.unwrap().to_string() == "blue" && digit > 14 {
                    //println!("in game {} we found: {} ,{}", game+1, idx, color.unwrap().to_string());
                    isPossible = false;
                    break;
                }
                if color.unwrap().to_string().eq("red") && digit > 12 {
                    //println!("in game {} we found: {} ,{}", game+1, idx, color.unwrap().to_string());
                    isPossible = false;
                    break;
                }       
                if color.unwrap().to_string() == "green" && digit > 13 {
                    //println!("in game {} we found: {} ,{}", game+1, idx, color.unwrap().to_string());
                    isPossible = false;
                    break;
                }       
            }
        }
        if isPossible {
            sum += (game as i32) + 1;
        }

    }
    println!("{}", sum);
    
}
