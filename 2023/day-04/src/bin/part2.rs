use aoc_utils_by_nifalu::RetrieveInts;


fn main() {
    let contents = include_str!("./input.txt");

    let mut total_scratchcards = 0;

    let mut games : Vec<(usize, i32)> = Vec::new();

    for (idx, line) in contents.lines().enumerate() {
        let mut wins: i32 = 0;

        let (_, data) = line.split_once(": ").unwrap();
        
        let (winning_numbers, own_numbers) = data.split_once(" | ").unwrap();

        let winning_numbers = winning_numbers.retrieve_all_ints();
        let own_numbers = own_numbers.retrieve_all_ints();

        for num in own_numbers {
            if winning_numbers.contains(&num) {
                wins += 1;
            }
        }

        games.push((idx, wins));
    }

    for (game, _) in games.iter() {
        //println!("Game {} has {} wins", game, wins);
        let tmp = count_scratch_cards(*game as i32, &games);
        total_scratchcards += tmp;
        //println!("Game {} added {} more Scratchcards", game, tmp);
    }
    

    println!("{}", total_scratchcards);
}


fn count_scratch_cards(game: i32, games: &Vec<(usize,i32)>) -> i32 {
    if games[game as usize].1 == 0 {
        return 1;
    }

    let mut tmp = 0;
    for i in 0..games[game as usize].1 {
        tmp += count_scratch_cards(game+i+1, games);
    }
    return tmp + 1;
}