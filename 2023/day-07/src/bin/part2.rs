use std::collections::HashMap;

use regex::Regex;
fn main() {
    let contents = include_str!("./lina.txt");

    let mut hand_and_bids: Vec<(String, i64)> = Vec::new();
    for line in contents.lines() {
        let (hand, bid) = line.split_once(" ").unwrap();
        hand_and_bids.push((hand_to_string(hand), bid.parse::<i64>().ok().unwrap()))
    }
    
    hand_and_bids.sort_by(|a,b| b.0.cmp(&a.0));

    let mut result: i64 = 0;

    let mut card_types: [i32; 7] = [0,0,0,0,0,0,0];

    for (rank, hand) in hand_and_bids.iter().enumerate() {
        result += (rank as i64 + 1) * hand.1;
        if hand.0.starts_with("A") {
            card_types[0] += 1;
        }
        if hand.0.starts_with("B") {
            card_types[1] += 1;
        }
        if hand.0.starts_with("C") {
            card_types[2] += 1;
        }
        if hand.0.starts_with("D") {
            card_types[3] += 1;
        }
        if hand.0.starts_with("E") {
            card_types[4] += 1;
        }
        if hand.0.starts_with("F") {
            card_types[5] += 1;
        }
        if hand.0.starts_with("G") {
            card_types[6] += 1;
        }
    }
    //println!("{:?}", hand_and_bids);
    println!("{}", result);
    println!("{:?}", card_types);
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
            "T" => "K",
            "9" => "L",
            "8" => "M",
            "7" => "N",
            "6" => "O",
            "5" => "P",
            "4" => "Q",
            "3" => "R",
            "2" => "S",
            "J" => "T",
            _ => ""
        }
    }).to_string())
}

fn parse_hand(line: String) -> String {
    let mut hands: HashMap<char, i64> = HashMap::new();
    
    // Count the hand
    for card in line.chars() {
        let num_of_cards = hands.get(&card).unwrap_or(&0);
        hands.insert(card, num_of_cards + 1);
    }

    // Find most common char
    let mut most_common_char = ('_', 0);
    for (card, i) in &hands {
        if most_common_char.1 < *i && !(*card).eq(&'T'){
            most_common_char = (*card, *i);
        }
    }
    // All jokers are now the most common char
    let joker_hand = line.replace("T", most_common_char.0.to_string().as_str());
    // Now count the hand again
    let mut joker_hands: HashMap<char, i64> = HashMap::new();
    for card in joker_hand.chars() {
        let num_of_cards = joker_hands.get(&card).unwrap_or(&0);
        joker_hands.insert(card, num_of_cards + 1);
    }

    
    for (_, amount) in &joker_hands {
        if amount == &5 {
            return format!("A{}", line); // Five of a Kind
        }
        if amount == &4 {
            return format!("B{}", line); // Four of a Kind
        }
        if (amount == &2 || amount == &3) && joker_hands.len() == 2 {
            return format!("C{}", line); // Full House
        }
        if amount == &3 {
            if !joker_hand.eq(&line) {
            }
            return format!("D{}", line); // Three of a Kind
        }
        if amount == &2 && joker_hands.len() == 3 {
            if !joker_hand.eq(&line) {
            }
            return format!("E{}", line); // Two Pair
        }
        if amount == &2 {
            return format!("F{}", line); // One Pair
        }
    }
    return format!("G{}", line); // High card
}