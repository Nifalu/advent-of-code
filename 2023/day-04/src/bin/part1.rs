use aoc_utils_by_nifalu::RetrieveInts;



fn main() {
    let contents = include_str!("./input.txt");

    let mut total_points = 0;

    for line in contents.lines() {
        let mut round_points: i32 = 0;

        let (_, data) = line.split_once(": ").unwrap();
        
        let (winning_numbers, own_numbers) = data.split_once(" | ").unwrap();

        let winning_numbers = winning_numbers.retrieve_all_ints();
        let own_numbers = own_numbers.retrieve_all_ints();

        for num in own_numbers {
            if winning_numbers.contains(&num) {
                //print!("{}, ", num);
                if round_points == 0 {
                    round_points = 1;
                } else {
                    round_points = round_points * 2;
                }
            }
        }
        //println!();
        //println!("{}", round_points);
        total_points += round_points;
    }

    println!("{}", total_points);
}
