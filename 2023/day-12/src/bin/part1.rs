
fn main() {

    //let mut springs: Vec<(Vec<char>, Vec<i64>)> = Vec::new();
    let mut springs: Vec<(&str, Vec<usize>)> = Vec::new();
    include_str!("./test.txt").lines()
    .for_each(|line: &str| {
        let (mut s, r) = line.split_once(" ").unwrap();
        let mut r: Vec<usize> = r.split(',').filter_map(|f| f.parse::<usize>().ok()).collect::<Vec<usize>>();

        s = s.trim_start_matches('.');
        s = s.trim_end_matches('.');

        while s.starts_with('#') || s.ends_with('#') {
            if s.starts_with('#') {
                s = &s[r[0]..];
                r.remove(0);
            }
            if s.ends_with('#') {
                s = &s[..(s.len()-r.last().unwrap())];
                r.remove(r.len()-1);
            }
            s = s.trim_start_matches('.');
            s = s.trim_end_matches('.');
        }
        springs.push((s, r))
    });



    for line in springs {
        println!("Springs: {:?}, Record: {:?}", line.0, line.1);
    }
}