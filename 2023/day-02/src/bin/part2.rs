use regex::Regex;

fn main() {
    let contents = include_str!("./input.txt");

    let re = Regex::new(r"[, ]").unwrap();

    for line in contents.lines() {
        let game: Vec<&str> = line.split(':').collect();
        let sets: Vec<&str> = (game.get(1).unwrap()).split(';').collect();
        
        for set in sets {
            let cubes: Vec<&str> = re.split(set).collect();
            
        }
    }
    
}
