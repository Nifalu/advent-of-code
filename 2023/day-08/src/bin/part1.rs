use std::collections::HashMap;

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

fn main() {
    let (directions, data) = include_str!("./input.txt").split_once("\n\n").unwrap();

    let mut direction = directions.chars();

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


    let mut current_node = graph.get("AAA").expect("AAA not found in graph");
    let mut steps = 0;
    while current_node.name != "ZZZ" {
        
        if let Some(direction_char) = direction.next() {
            steps += 1;
            current_node = match direction_char {
                'L' => graph.get(current_node.l.as_ref().expect("Left child not found")).expect("Node not found in graph"),
                'R' => graph.get(current_node.r.as_ref().expect("Right child not found")).expect("Node not found in graph"),
                _ => panic!("Invalid direction character"),
            };
        } else {
            direction = directions.chars();
        }
    }

    println!("Steps: {}", steps);
}