use std::{collections::HashMap, fmt};

#[derive(Debug)] // Deriving Debug trait
struct Node {
    name: String,
    l: Option<String>,
    r: Option<String>,
}

impl Node {
    fn new(name: String) -> Node {
        Node {
            name,
            l: None,
            r: None,
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn main() {
    let (directions, data) = include_str!("./input.txt").split_once("\n\n").unwrap();

    let mut direction = directions.chars().cycle();

    let mut graph: HashMap<String, Node> = HashMap::new();

    // Fill the graph
    for n in data.lines(){
        let value = n.get(0..3).unwrap();
        let left = n.get(7..10).unwrap();
        let right = n.get(12..15).unwrap();

        graph.entry(value.to_string()).or_insert_with(|| Node::new(value.to_string()));
        graph.entry(left.to_string()).or_insert_with(|| Node::new(left.to_string()));
        graph.entry(right.to_string()).or_insert_with(|| Node::new(right.to_string()));
        
        graph.entry(value.to_string()).and_modify(|n| n.l = Some(left.to_string()));
        graph.entry(value.to_string()).and_modify(|n| n.r = Some(right.to_string()));
    }


    // Find starting Nodes
    let mut current_nodes: Vec<&Node> = graph.values()
    .filter(|node| node.name.ends_with("A")).collect();

    println!("There are {} starting points", {current_nodes.len()});

    let mut steps_to_z: Vec<i64> = Vec::new();

    let mut steps = 0;
    while let Some(direction_char) = direction.next() {
        steps += 1;
        let mut next_nodes = Vec::new();
        for node in &current_nodes {
            match direction_char {
                'L' => {
                    if let Some(left) = &node.l {
                        next_nodes.push(graph.get(left).expect("Node not found in graph"));
                    }
                },
                'R' => {
                    if let Some(right) = &node.r {
                        next_nodes.push(graph.get(right).expect("Node not found in graph"));
                    }
                },
                _ => panic!("Invalid direction character"),
            };
        }

        current_nodes = next_nodes;
        let mut finished = false;
        for node in &current_nodes {
            if node.name.ends_with("Z") {
                steps_to_z.push(steps);
            }
            if steps_to_z.len() == current_nodes.len() {
                finished = true;
            }
        }
        if finished {
            println!("{:?}", steps_to_z);

            let result = steps_to_z.iter().fold(1, |acc, &num| lcm(acc, num));
            println!("{}", result);
            return
        }
    }

    println!("Steps: {}", steps);
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a.abs() / gcd(a, b) * b.abs()
}