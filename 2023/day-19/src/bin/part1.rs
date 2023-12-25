use std::collections::{HashMap, HashSet};

fn main() {
    let (items, workflows) = parse_input(include_str!("./input.txt"));

    //println!("items\n{:?}\n\nworkflows{:?}", items, workflows);

    let mut accepted: HashSet<Item> = HashSet::new();

    for item in items {
        if apply_workflow(&item, workflows.get("in").expect("workflow exists"), &workflows) {
            //println!("{:?} was accepted", &item);
            accepted.insert(item);
        } else {
            //println!("{:?} was rejected", &item);
        }

    }

    let mut total: usize = 0;

    for item in accepted {
        total += item.x + item.m + item.a + item.s
    }

    println!("total: {}", total);

    //println!("accepted: \n {:?}", accepted);
}


fn parse_input(input: &str) -> (Vec<Item>, HashMap<&str, Vec<&str>>) {
    let (workflows, items) = input.split_once("\n\n").unwrap();
    let mut all_workflows: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut all_items: Vec<Item> = Vec::new();
    let (mut key, mut rule): (&str, &str);
    let mut rules: Vec<&str>;
    for line in workflows.lines() {
        (key, rule) = line.split_once("{").unwrap();
        rule = rule.trim_end_matches("}");
        rules = rule.split(",").collect();
        all_workflows.insert(key, rules);
    }
    


    for mut item in items.lines() {
        item = item.trim_start_matches("{");
        item = item.trim_end_matches("}");
        let values: Vec<&str> = item.split(",").collect();
        all_items.push( Item {
            x: values[0][2..].parse().expect("expect digit"),
            m: values[1][2..].parse().expect("expect digit"),
            a: values[2][2..].parse().expect("expect digit"),
            s: values[3][2..].parse().expect("expect digit"),
         })
    }

    (all_items, all_workflows)
}


fn apply_workflow(item: &Item, workflow: &Vec<&str>, all_workflows: &HashMap<&str, Vec<&str>>) -> bool {

    //println!("workflow: {:?}", workflow);
    for rule in workflow {
        let item_value: usize;
        let mut comp_value: usize = 0;
        let condition: bool;
        let chars: Vec<char> = rule.chars().collect();

        // Case: "A" or "R"
        if chars[0].eq(&'A') {
            println!("finally accept");
            return true;
        } else if chars[0].eq(&'R') {
            println!("finally reject");
            return false;
        }

        // Case: "item<value:target"
        if chars[1].eq(&'<') || chars[1].eq(&'>') {
            match chars[0] {
                'x' => item_value = item.x,
                'a' => item_value = item.a,
                'm' => item_value = item.m,
                's' => item_value = item.s,
                _ => panic!("should not be possible"),
            }

            let mut i = 2;
            while chars[i].ne(&':') {
                match chars[i] {
                    '0'..='9' => {
                        comp_value = comp_value * 10 + chars[i].to_digit(10).unwrap() as usize;
                    }
                    other => panic!("Unexpected character sequence... found: {} in workflow {:?}\nitem_value: {}\noperation: {}", other, workflow, item_value, chars[1]),
                }
                i += 1;
            }
            match chars[1] {
                '<' => condition = item_value < comp_value,
                '>' => condition = item_value > comp_value,
                _ => panic!("No valid operation")
            }

            i += 1; // jump over the ':'
            if condition {
                if chars[i].eq(&'A') {
                    println!("{} is {} than {} -> accept", item_value, chars[1], comp_value);
                    return true;
                } else if chars[i].eq(&'R') {
                    println!("{} is {} than {} -> reject", item_value, chars[1], comp_value);
                    return false;
                }
                let mut target = "".to_string();
                while i < chars.len() {
                    target.push(chars[i]);
                    i += 1;
                }
                println!("{} is {} than {} -> jump to {}", item_value, chars[1], comp_value, target);
                return apply_workflow(&item, all_workflows.get(target.as_str()).expect(format!("Expected a workflow, got op:{}, target:{}, x:{}, workflow:{:?}", chars[1], target, item.x, workflow).as_str()), all_workflows);
            }

        // Case "A" or "R" at the end of a workflow
        } else {
            let mut i = 0;
            let mut target = "".to_string();
            while i < chars.len() {
                target.push(chars[i]);
                i+=1;
            }
            return apply_workflow(&item, all_workflows.get(target.as_str()).expect(format!("Expected a workflow, got {}, {}, {:?}", target, item.x, workflow).as_str()), all_workflows);
        }
    }
    panic!("this should never be reached...");
}
        //println!(""); 

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Item {
    x: usize,
    a: usize,
    m: usize,
    s: usize,
}
