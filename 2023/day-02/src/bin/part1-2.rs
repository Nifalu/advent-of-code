fn main() {
    let contents = include_str!("./input.txt");

    let mut sum: i32 = 0;
    let mut power: i32 = 0;

    for (id,line) in contents.lines().enumerate() {

        // Separate the data from the game id, then split the data into the different rounds
        let (_, data) = line.split_once(": ").unwrap();
        let game: Vec<&str> = data.split("; ").collect();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        // For each cube in each round, retrieve the color and the amound.
        for round in game {
            for cube in round.split(", ") {
                let (amount, color) = cube.split_once(" ").unwrap();
                let amount = amount.parse::<i32>().unwrap_or(0);

                // We only care about largest number associated with a color...
                if color == "red" {
                    red = red.max(amount);
                }
                if color == "green" {
                    green = green.max(amount);
                }
                if color == "blue" {
                    blue = blue.max(amount);
                }
            }
        }

        // Check if game was possible with the given amount of cubes
        if red <= 12 && green <= 13 && blue <= 14 {
            sum += id as i32 + 1;
        }
        // Calculate the least amount of cubes needed for the game
        power += red * green * blue;
    }

    println!("Part 1: Sum of all 'possible' games: {}", sum);
    println!("Part 2: Sum of the power of all games: {}", power);
    
}
