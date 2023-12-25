use std::collections::HashMap;


#[derive(Clone)]
pub enum Node {
    Conjunction(Conjunction),
    FlipFlop(FlipFlop),
}

impl Node {
    // Method to process a pulse based on the type of Node
    pub fn push(&mut self,sender: String, pulse: bool) {
        match self {
            Node::Conjunction(conjunction) => conjunction.push(sender, pulse),
            Node::FlipFlop(flip_flop) => flip_flop.push(pulse),
        }
    }

    // Method to send the value based on the type of Node
    pub fn send(&mut self, modules: &mut std::collections::HashMap<String, Node> ) {
        match self {
            Node::Conjunction(conjunction) => conjunction.send(modules),
            Node::FlipFlop(flip_flop) => flip_flop.send(modules),
        }
    }

    pub fn get_statistics(&self) -> (usize, usize) {
        match self {
            Node::Conjunction(conjunction) => conjunction.get_statistics(),
            Node::FlipFlop(flip_flop) => flip_flop.get_statistics(),
        }
    }

    pub fn get_id(&self) -> String {
        match self {
            Node::Conjunction(conjunction) => conjunction.get_id(),
            Node::FlipFlop(flip_flop) => flip_flop.get_id(),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Node::Conjunction(a), Node::Conjunction(b)) => a == b,
            (Node::FlipFlop(a), Node::FlipFlop(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Node {}


#[derive(Clone)]
pub struct Conjunction {
    id: String,
    memory: HashMap<String, bool>,
    receivers: Vec<String>,
    pub lsent: usize,
    pub hsent: usize,
}

impl Conjunction {
    pub fn new(id: String, receivers: Vec<String>) -> Self {
        Self { id, memory: HashMap::new(), receivers, lsent: 0, hsent: 0}
    }

    pub fn push(&mut self,sender: String ,pulse: bool) {
        self.memory.insert(sender, pulse);
    }

    pub fn send(&mut self, modules: &mut std::collections::HashMap<String, Node>) {
        let pulse = self.memory.values().any(|v| v == &false);
        for key in &self.receivers {
            let receiver = modules.get_mut(key);
            if receiver.is_some() {
                let r = receiver.unwrap();
                    r.push(self.id.clone(), pulse);
                    println!("{} sends {} to {}", self.id, pulse, r.get_id())
            }
            if pulse {
                self.hsent += 1;
            } else {
                self.lsent += 1;
            }
        }
        self.memory.clear();
    }

    pub fn get_statistics(&self) -> (usize, usize) {
        (self.lsent, self.hsent)
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}

impl PartialEq for Conjunction {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Conjunction {}

#[derive(Clone)]
pub struct FlipFlop {
    id: String,
    value: bool,
    send: bool,
    receivers: Vec<String>,
    pub lsent: usize,
    pub hsent: usize,
}

impl FlipFlop {
    pub fn new(id: String, receivers: Vec<String>) -> Self {
        Self { id, value: false, send: false, receivers, lsent: 0, hsent: 0}
    }

    pub fn push(&mut self, pulse: bool) {
        if !pulse {
            self.value = !self.value;
            self.send = true;
        }
    }

    pub fn send(&mut self, modules: &mut std::collections::HashMap<String, Node>) {
        if self.send {
            for key in &self.receivers {
                let receiver = modules.get_mut(key);
                if receiver.is_some() {
                    let r = receiver.unwrap();
                    r.push(self.id.clone(), self.value);
                    println!("{} sends {} to {}", self.id, self.value, r.get_id())
                }
                if self.value {
                    self.hsent += 1;
                } else {
                    self.lsent += 1;
                }
            }
        }
    }

    pub fn get_statistics(&self) -> (usize, usize) {
        (self.lsent, self.hsent)
    }

    pub fn get_id(&self) -> String {
        self.id.clone()
    }
    
}

impl PartialEq for FlipFlop {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for FlipFlop {}