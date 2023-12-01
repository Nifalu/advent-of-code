use regex::Regex;


fn main() {
    let contents = include_str!("./input.txt");
    // println!("{}", contents);
    let re = Regex::new(r"\d").unwrap();

    let mut sum: i32 = 0;

    for line in contents.lines() {
        let digit_positions: Vec<_> = re.find_iter(line).collect();

        if !digit_positions.is_empty() {
            sum += digit_positions.first().unwrap().as_str().parse::<i32>().unwrap() * 10;
            sum += digit_positions.last().unwrap().as_str().parse::<i32>().unwrap();
        }
    }

    println!("{}", sum);
}