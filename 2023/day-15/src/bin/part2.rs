use regex::Regex;
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Boxx {
    pub lenses: IndexMap<String, i32>,
}

impl Boxx {
    // Constructor
    pub fn new() -> Boxx {
        Boxx {
            lenses: IndexMap::new(),
        }
    }

    // A method that mutates the struct
    pub fn insert_lense(&mut self, key: String, value: &str) {
        match value.parse::<i32>() {
            Ok(parsed_value) => self.lenses.insert(key, parsed_value),
            Err(_) => panic!("Lense value is not a Number..."), // or handle the error more gracefully
        };
    }

    pub fn remove_lense(&mut self, key: &str) {
        self.lenses.shift_remove(key);
    }
}


fn main() {
    
    let sub_strings: Vec<&str> = include_str!("./input.txt").split(',').collect::<Vec<&str>>();
    let re: Regex = Regex::new(r"[=-]").unwrap();
    let mut boxxes: IndexMap<i32, Boxx> = IndexMap::new();

    for s in sub_strings {
        let splitted: Vec<&str> = re.split(s).collect();
        let mut curr_v: i32 = 0;
        splitted[0].chars()
        .for_each(|c| {
            curr_v += c as i32;
            curr_v *= 17;
            curr_v = curr_v % 256;
        });
        
        let boxx = boxxes.entry(curr_v).or_insert(Boxx::new());

        if s.contains('=') {
            boxx.insert_lense(splitted[0].to_string(), splitted[1]);
        }

        if s.contains('-') {
            boxx.remove_lense(splitted[0]);
        }
    }

    let mut total_power: i32 = 0;
    for (boxx_idx, boxx) in boxxes {
        for (lense_idx, (_, focal_length)) in boxx.lenses.iter().enumerate() {
            total_power += (boxx_idx + 1) * (lense_idx as i32 + 1) * focal_length;
        }
        
        println!("Box: {}, Lenses: {:?}, Power: {}",boxx_idx, boxx.lenses.values(), total_power);
    }
}