use template::{Node, FlipFlop, Conjunction};
use std::collections::{HashMap, VecDeque};

const BUTTON_PRESSES: usize = 1000;

fn main() {
    let input = include_str!("./test.txt");

    let mut modules: HashMap<String, Node> = HashMap::new();

    let mut broadcaster: Vec<String> = Vec::new();

    for line in input.lines() {
        let mut receivers: Vec<String> = Vec::new();
        let (prefix, r_list) = line.split_once(" -> ").unwrap();
        r_list.split(", ").for_each(|s: &str| receivers.push(s.to_string()));
        let key = &prefix[1..];
        if prefix.starts_with("%") {
            modules.insert(key.to_string(), Node::FlipFlop(FlipFlop::new(key.to_string(), receivers)));
        } else if prefix.starts_with("&") {
            modules.insert(key.to_string(), Node::Conjunction(Conjunction::new(key.to_string(), receivers)));
        } else if prefix.starts_with("broadcaster") {
            broadcaster = receivers;
        } else {
            panic!("should never be here");
        }
    }

    let mut pulses: (usize, usize) = (0, 0);


    let mut queue: VecDeque<String> = VecDeque::new();


    for i in 0..1{

        // Setup queue with starting receivers
        queue.clear();
        for key in &broadcaster {
            let node = modules.get_mut(key).expect("???");
            node.push("broadcaster".to_string(), true);
            queue.push_back(key.to_string());
        }

        while !queue.is_empty() {
            //println!("elements in queue: {}", queue.len());
            let key = queue.pop_front().expect("queue should not be empty here...");
            // Temporarily take the node out of the modules
            if let Some(mut node) = modules.remove(&key) {
                node.send(&mut modules);
                
                // Add receiving nodes to the queue
                for receiver in get_receivers(&key, input) {
                    queue.push_back(receiver);
                }

                // Put the node back into modules
                modules.insert(key, node);
            }
            
        }

        for (_, node) in &modules {
            let (low, high) = node.get_statistics();
            pulses.0 += low;
            pulses.1 += high;
        }

        println!("button press: {}, pulses: {:?}", i, pulses);
    }


    fn get_receivers(id: &str, input: &str) -> Vec<String> {
        let mut receivers: Vec<String> = Vec::new();
        for line in input.lines() {
            let (prefix, r_list) = line.split_once(" -> ").unwrap();
            let prefix = &prefix[1..];
            if prefix.starts_with(id) {
                r_list.split(", ").for_each(|s: &str| receivers.push(s.to_string()));
                break;
            }
        }
        receivers
    }


    for module in modules {
        println!("{}", module.0);
    }
}