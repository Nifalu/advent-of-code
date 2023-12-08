use std::collections::HashMap;

use regex::Regex;
fn main() {
    let contents = include_str!("./input.txt");

    let mut hand_and_bids: Vec<(String, i64)> = Vec::new();
    for line in contents.lines() {
        let (hand, bid) = line.split_once(" ").unwrap();
        hand_and_bids.push((hand_to_string(hand), bid.parse::<i64>().ok().unwrap()))
    }
    
    hand_and_bids.sort_by(|a,b| b.0.cmp(&a.0));

    let mut result: i64 = 0;

    for (rank, hand) in hand_and_bids.iter().enumerate() {
        result += (rank as i64 + 1) * hand.1;
    }

    println!("{:?}", result);
}

// abcdefg - hijklmnopqrst
//           akqjt98765432

fn hand_to_string(hand: &str) -> String {
    let re = Regex::new(r"(A|K|Q|J|T|9|8|7|6|5|4|3|2)").unwrap();
    parse_hand(re.replace_all(hand, |caps: &regex::Captures| {
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
    }).to_string())
}

fn parse_hand(line: String) -> String {
    let mut hands: HashMap<char, i64> = HashMap::new();

    for card in line.chars() {
        let num_of_cards = hands.get(&card).unwrap_or(&0);
        hands.insert(card, num_of_cards + 1);
    }

    for (_, amount) in &hands {
        if amount == &5 {
            return format!("A{}", line); // Five of a Kind
        }
        if amount == &4 {
            return format!("B{}", line); // Four of a Kind
        }
        if (amount == &2 || amount == &3) && hands.len() == 2 {
            return format!("C{}", line); // Full House
        }
        if amount == &3 {
            return format!("D{}", line); // Three of a Kind
        }
        if amount == &2 && hands.len() == 3 {
            return format!("E{}", line); // Two Pair
        }
        if amount == &2 {
            return format!("F{}", line); // One Pair
        }
    }
    return format!("G{}", line); // High card
}