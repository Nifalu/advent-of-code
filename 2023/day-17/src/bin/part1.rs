use std::collections::{BinaryHeap, HashSet};
use std::cmp::{Reverse, Ordering};

fn main() {
    let mut map = parse_to_vec_vec_i8(include_str!("./input.txt"));

    let start:(usize, usize) = (0, 0);
    let end: (usize, usize) = (map.len()-1, map[0].len()-1);

    let mut next_nodes: BinaryHeap<Node> = BinaryHeap::new();

    let mut visited: HashSet<Node> = HashSet::new();

    let mut curr_node: Node;

    next_nodes.push(Node { coord: start, prev: start, straights: 0 as usize , heat_loss: map[start.0][start.1] });

    while !next_nodes.is_empty() {
        curr_node = next_nodes.pop().expect("Expect Node");

        if curr_node.coord == end {
            solution(&curr_node);
        }

        visited.insert(curr_node);

        for neighbor in get_neighbors(&curr_node) {

        }

    }

}

fn get_neighbors(node: &Node) -> Vec<Node> {
    let neighbors: Vec<Node> = Vec::new();
    neighbors
}

fn solution(node: &Node) {
    todo!();
}


#[derive(Eq, PartialEq, Hash)]
struct Node {
    coord: (usize, usize),
    prev: (usize, usize),
    straights: usize,
    heat_loss: i8,
}

// Implement PartialOrd by manually defining how to compare two nodes
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.heat_loss.cmp(&other.heat_loss))
    }
}

// Implement Ord by manually defining how to compare two nodes
// Here, we want to compare based on cost primarily
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the order for a min-heap (lowest cost first)
        other.heat_loss.cmp(&self.heat_loss)
    }
}


fn parse_to_vec_vec_i8(input: &str) -> Vec<Vec<i8>> {
    input
        .lines() // Splits the string into lines
        .map(|line| {
            line.chars() // Iterates over characters in a line
                .map(|c| c.to_digit(10).expect("Expect digit") as i8) // Converts each character to i8
                .collect() // Collects into Vec<i8>
        })
        .collect() // Collects all Vec<i8> into Vec<Vec<i8>>
}