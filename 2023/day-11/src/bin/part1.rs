use aoc_utils_by_nifalu::RetrieveSubstring;


fn main() {
    let raw_input = include_str!("./input.txt");

    let mut space: Vec<Vec<char>> = Vec::new();
    raw_input.lines()
    .for_each(|line: &str| {
        space.push(line.chars().collect::<Vec<char>>())
    });

    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    raw_input.lines().enumerate().for_each(
        |(line_idx, l)| l.chars().enumerate().for_each(
            |(char_idx, c)| 
                if c.ne(&'#') {
                    galaxies.push((line_idx, char_idx));
                } 
        )
    );

    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {

        }
    }

}


fn count_deep_space((x, y), (dx, dy)) -> usize {
}