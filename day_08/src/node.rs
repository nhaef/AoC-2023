use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Node {
    pub label: [u8; 3],
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(label: [u8; 3]) -> Self {
        Self {
            label,
            left: None,
            right: None,
        }
    }
    pub fn left(&self) -> Rc<RefCell<Node>> {
        self.left.as_ref().unwrap().clone()
    }
    pub fn right(&self) -> Rc<RefCell<Node>> {
        self.right.as_ref().unwrap().clone()
    }
}

#[allow(dead_code)]
pub fn get_instructions_and_label_to_node_map(
    input: &str,
) -> (&str, HashMap<&str, Rc<RefCell<Node>>>) {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .expect(&format!("Failed to read first line of input {}", input));
    lines
        .next()
        .expect(&format!("Failed to skip second line of input {}", input));

    let mut label_to_node: HashMap<&str, Rc<RefCell<Node>>> = HashMap::new();
    for line in lines {
        let label = &line[..3];
        let label_left = &line[7..10];
        let label_right = &line[12..15];

        let this_node = match label_to_node.get_mut(label) {
            Some(this_node) => this_node.clone(),
            None => {
                let this_node_new = Rc::new(RefCell::new(Node::new(
                    label.as_bytes().try_into().unwrap(),
                )));
                label_to_node.insert(label, this_node_new.clone());
                this_node_new
            }
        };

        this_node.borrow_mut().left = match label_to_node.get(label_left) {
            Some(left_node) => Some(left_node.clone()),
            None => {
                let left_node_new = Rc::new(RefCell::new(Node::new(
                    label_left.as_bytes().try_into().unwrap(),
                )));
                label_to_node.insert(label_left, left_node_new.clone());
                Some(left_node_new)
            }
        };

        this_node.borrow_mut().right = match label_to_node.get(label_right) {
            Some(right_node) => Some(right_node.clone()),
            None => {
                let right_node_new = Rc::new(RefCell::new(Node::new(
                    label_right.as_bytes().try_into().unwrap(),
                )));
                label_to_node.insert(label_right, right_node_new.clone());
                Some(right_node_new)
            }
        };
    }

    (instructions, label_to_node)
}

#[allow(dead_code)]
pub fn find_aaa_to_zzz_steps(
    (instructions, label_to_node_map): (&str, HashMap<&str, Rc<RefCell<Node>>>),
) -> usize {
    let mut current_node = label_to_node_map
        .get("AAA")
        .expect("Failed to find node AAA")
        .clone();
    let final_label = "ZZZ".as_bytes();
    for (step, instruction) in instructions.chars().cycle().enumerate() {
        match instruction {
            'L' => {
                let next_node = current_node.borrow().left();
                current_node = next_node;
            }
            'R' => {
                let next_node = current_node.borrow().right();
                current_node = next_node;
            }
            _ => panic!("Unexpected instruction {}", instruction),
        }
        if current_node.borrow().label == final_label {
            return step + 1;
        }
    }
    unreachable!()
}

#[allow(dead_code)]
pub fn find_a_to_z_steps(
    (instructions, label_to_node_map): (&str, HashMap<&str, Rc<RefCell<Node>>>),
) -> usize {
    let start_byte = 'A' as u8;
    let end_byte = 'Z' as u8;
    let mut current_nodes = label_to_node_map
        .into_values()
        .filter(|node| node.borrow().label[2] == start_byte)
        .collect::<Vec<Rc<RefCell<Node>>>>();

    for (step, instruction) in instructions.chars().cycle().enumerate() {
        let mut done = true;
        match instruction {
            'L' => {
                for node in current_nodes.iter_mut() {
                    let next = node.borrow().left();
                    done = done && next.borrow().label[2] == end_byte;
                    *node = next;
                }
            }
            'R' => {
                for node in current_nodes.iter_mut() {
                    let next = node.borrow().right();
                    done = done && next.borrow().label[2] == end_byte;
                    *node = next;
                }
            }
            _ => panic!("Unexpected instruction {}", instruction),
        }
        if done {
            return step + 1;
        }
    }
    unreachable!()
}
