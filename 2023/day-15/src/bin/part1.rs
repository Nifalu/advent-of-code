
fn main() {
    
    let sub_strings = include_str!("./input.txt").split(',').collect::<Vec<&str>>();

    let mut total_v: i32 = 0;
    for s in sub_strings {
        let mut curr_v: i32 = 0;
        s.chars()
        .for_each(|c| {
            curr_v += c as i32;
            //print!("ASCII: {}, ", curr_v);
            curr_v *= 17;
            curr_v = curr_v % 256;
            //println!("{}", curr_v);
        });
        total_v += curr_v;
        println!("{}",curr_v);
    }
    println!("{}",total_v);

}