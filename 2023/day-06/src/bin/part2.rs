
fn main() {
    let mut contents:std::str::Lines<'_> = include_str!("./input.txt").lines();

    let times: Vec<f64> = contents
    .next()
    .unwrap()
    .split_whitespace()
        .filter_map(|num| num.parse::<f64>().ok())
        .collect();

    let distances: Vec<f64> = contents
    .next()
    .unwrap()
    .split_whitespace()
        .filter_map(|num| num.parse::<f64>().ok())
        .collect();

    let time: f64 = times
        .iter()
        .map(|&num| num.to_string())
        .collect::<Vec<_>>()
        .join("").parse().unwrap_or(0.0);

    let distance: f64 = distances
        .iter()
        .map(|&num| num.to_string())
        .collect::<Vec<_>>()
        .join("").parse().unwrap_or(0.0);

    let x: f64 = (((time*time - 4.0*(distance)).sqrt() - time) / -2.0 ).floor() + 1.0;
    let range = time - 2.0*x + 1.0; 
    
    println!("Produkt: {:?}", range);
}
 