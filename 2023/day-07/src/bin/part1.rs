use core::num;
use std::collections::HashMap;

use regex::Regex;
fn main() {
    let contents = include_str!("./input.txt");

    let mut hand_and_bids: Vec<(String, i64)> = Vec::new();
    for line in contents.lines() {
        let (hand, bid) = line.split_once(" ").unwrap();
        hand_and_bids.push((hand_to_string(hand), bid.parse::<i64>().ok().unwrap()))
    }
    
}

// abcdefg - hijklmnopqrst
//           akqjt98765432

fn hand_to_string(hand: &str) -> String {
    let re = Regex::new(r"(quick|brown|fox)").unwrap();
    return re.replace_all(hand, |caps: &regex::Captures| {
        match caps.get(0).map_or("", |m| m.as_str()) {
            "A" => "H",
            "K" => "I",
            "Q" => "J",
            "J" => "K",
            "T" => "L",
            "9" => "M",
            "8" => "N",
            "7" => "O",
            "6" => "P",
            "5" => "Q",
            "4" => "R",
            "3" => "S",
            "2" => "T",
            _ => ""
        }
    }).to_string()
}

fn parse_hand(hand: &str) -> &str {
    let mut hands: HashMap<char, i64> = HashMap::new();

    for card in hand.chars() {
        let num_of_cards = hands.get(&card).unwrap_or(&0);
        hands.insert(card, num_of_cards + 1);
    }

    if hands.len() == 1 {
        return "A" + hand;
    }

    "s"
}