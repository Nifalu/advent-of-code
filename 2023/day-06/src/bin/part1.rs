
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

    let mut range: f64 = 1.0;
    for (idx, t) in times.iter().enumerate() {
        let x: f64 = (((t*t - 4.0*(distances[idx])).sqrt() - *t) / -2.0 ).floor() + 1.0;
        println!("{}", x);
        range = range * (t - 2.0*x + 1.0); 
    }
    println!("Produkt: {:?}", range);
}
 